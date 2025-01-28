import enum
import typing


class SquareState(enum.Enum):
    EMPTY = 0
    X = enum.auto()
    O = enum.auto()  # noqa: E741

    def __str__(self) -> str:
        match self:
            case SquareState.EMPTY:
                return " "
            case SquareState.X:
                return "X"
            case SquareState.O:
                return "O"


class GameResult(enum.Enum):
    DRAW = enum.auto()
    X_WINS = enum.auto()
    O_WINS = enum.auto()


class GameState:
    def __init__(self):
        self.board = [[SquareState.EMPTY] * 3 for _ in range(3)]

    def __str__(self) -> str:
        ret_lines = []
        for row_idx, row in enumerate(self.board):
            line = ""
            for col_idx, square in enumerate(row):
                if col_idx:
                    line += "|"
                line += str(square)
            ret_lines.append(line)
            if row_idx < len(self.board) - 1:
                ret_lines.append("-+-+-")
        return "\n".join(ret_lines)

    def check_result(self) -> GameResult | None:
        for row in self.board:
            if all(square == SquareState.X for square in row):
                return GameResult.X_WINS
            if all(square == SquareState.O for square in row):
                return GameResult.O_WINS
        for col_idx in range(len(self.board[0])):
            if all(row[col_idx] == SquareState.X for row in self.board):
                return GameResult.X_WINS
            if all(row[col_idx] == SquareState.O for row in self.board):
                return GameResult.O_WINS
        diags = (
            (self.board[0][0], self.board[1][1], self.board[2][2]),
            (self.board[0][2], self.board[1][1], self.board[2][0]),
        )
        for diag in diags:
            if all(square == SquareState.X for square in diag):
                return GameResult.X_WINS
            if all(square == SquareState.O for square in diag):
                return GameResult.O_WINS
        if all(all(square != SquareState.EMPTY for square in row) for row in self.board):
            return GameResult.DRAW
        return None


class Player(typing.Protocol):
    def make_move(self, game_state: GameState) -> tuple[int, int]: ...

    @property
    def symbol(self) -> SquareState: ...

    # TODO: empty symbol doesn't make sense. More exact type would be good


class Human:
    def __init__(self, symbol: SquareState) -> None:
        self.symbol = symbol

    def make_move(self, game_state: GameState) -> tuple[int, int]:
        print(f"Player {self.symbol} turn. Enter row and column in [0-2], separated by space:")
        while True:
            player_input = input()
            coord_strs = player_input.split()
            if len(coord_strs) != 2:
                print("please enter 2 numbers separated by space")
                continue
            coords = []
            try:
                for coord_str in coord_strs:
                    coord = int(coord_str)
                    if coord < 0 or coord > 2:
                        raise ValueError()
                    coords.append(coord)
            except ValueError:
                print("Invalid row or column. Must be 0, 1, or 2")
                continue
            if game_state.board[coords[0]][coords[1]] != SquareState.EMPTY:
                print("Position is already taken. Try again.")
                continue
            return tuple(coords)


def main():
    gs = GameState()
    while gs.check_result() is None:
        print(gs)
        players = Human(SquareState.X), Human(SquareState.O)
        for player in players:
            coords = player.make_move(gs)
            gs.board[coords[0]][coords[1]] = player.symbol
            print(gs)
            result = gs.check_result()
            if result is not None:
                match result:
                    case GameResult.DRAW:
                        print("Draw")
                    case GameResult.X_WINS:
                        print("X wins")
                    case GameResult.O_WINS:
                        print("O wins")
                break


if __name__ == "__main__":
    main()
