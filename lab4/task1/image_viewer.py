import subprocess
from file_viewer import FileViewer


class ImageViewer(FileViewer):
    def __init__(self, path: str):  # pylint: disable=useless-super-delegation
        super().__init__(path)

    def view(self):
        subprocess.run(['mspaint.exe', self.path], check=True)
