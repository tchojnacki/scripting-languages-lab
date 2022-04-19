import mimetypes
from image_viewer import ImageViewer
from text_viewer import TextViewer


class ViewerCreator:
    def __init__(self):  # pylint: disable=useless-super-delegation
        super().__init__()

    def create_viewer(self, path: str):
        return self._detect_viewer_type(path)(path)

    def _detect_viewer_type(self, path: str):
        match mimetypes.guess_type(path)[0].partition('/')[0]:
            case 'text':
                return TextViewer
            case 'image':
                return ImageViewer
            case _:
                raise ValueError('Invalid file type.')
