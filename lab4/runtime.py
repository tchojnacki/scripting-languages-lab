from viewer_creator import ViewerCreator
from multiple_accumulate import MultipleAccumulate


class Runtime:
    @staticmethod
    def test_duck_typing():
        creator = ViewerCreator()
        viewer = creator.create_viewer(r'test_files\tadeusz.txt')

        acc = MultipleAccumulate(
            [1, 2, 3],
            lambda a, b: a + b,
            min,
        )

        Runtime.print_viewer_stats(viewer)
        Runtime.print_viewer_stats(acc)

    @staticmethod
    def print_viewer_stats(text_viewer):
        print(text_viewer.get_data())
