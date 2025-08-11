import lib_sudoku as sudoku


def tests():
    reader = sudoku.PuzzleReader("puzzles.csv")

    sudoku.async_speedtest(reader)
    sudoku.synchronous_speedtest(reader)


if __name__ == "__main__":
    tests()
