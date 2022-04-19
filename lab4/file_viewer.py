from abc import ABC, abstractmethod


class FileViewer(ABC):
    '''
    Abstract base class representing a file viewer.
    It should be subtyped by classes representing concrete file types e.g. ImageViewer.
    '''

    def __init__(self, path: str):
        super().__init__()
        self.path = path

    @abstractmethod
    def view(self):
        '''
        Opens the given file for viewing in an appropriate built-in program.
        '''
