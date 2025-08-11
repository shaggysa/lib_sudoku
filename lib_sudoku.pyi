from .lib_sudoku import *
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    class PuzzleReader:
        def __init__(self, filename: str) -> PuzzleReader:
            """
            Takes in the name of a csv file containing sudoku puzzles in the format "puzzle,solution" and returns a PuzzleReader object.

            Args:
                filename (str): The name of the csv file.

            Returns:
                PuzzleReader object.

            Raises:
                FileNotFoundError: If the file does not exist.
                ValueError: If the file is malformed.
            """
            ...

    def synchronous_speedtest(puzzle_reader: PuzzleReader, verbose: bool):
        """
        Takes in a puzzle reader object and solves all the puzzles. It will also check all the solutions to confirm that the solver is current.

        Args:
            puzzle_reader (PuzzleReader): The puzzle reader object.
            verbose (bool, optional): Whether the function should print all unsolved and solved puzzles. This is good for debugging, but it adds a lot of overhead. Defaults to False.
        """
        ...

    def async_speedtest(puzzle_reader: PuzzleReader, verbose: bool):
        """
        Takes in a puzzle reader object and solves all the puzzles with multithreading. It should be much faster. The function will also check all the solutions to confirm that the solver is current.

        Args:
            puzzle_reader (PuzzleReader): The puzzle reader object.
            verbose (bool, optional): Whether the function should print all unsolved and solved puzzles. This is good for debugging, but it adds a lot of overhead. Defaults to False.
        """
        ...
