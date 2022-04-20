from viewer_creator import ViewerCreator
from multiple_accumulate import MultipleAccumulate
from runtime import Runtime


def test_task1():
    print('TASK 1')

    creator = ViewerCreator()

    image_viewer = creator.create_viewer(r'test_files\samoyed.jpg')
    image_viewer.view()

    text_viewer = creator.create_viewer(r'test_files\tadeusz.txt')
    text_viewer.view()

    stats = text_viewer.get_data()
    print(f'lines: {stats.number_of_lines}')
    print(f'words: {stats.number_of_words}')
    print(f'nonalpha: {stats.number_of_nonalpha}')


def test_task2():
    print('TASK 2')

    def my_sum(first, second):
        return first + second

    acc = MultipleAccumulate(
        [0, 2, 3.14, 5, 1],
        my_sum,  # named function
        max,  # built-in function
        # multiple anonymous functions
        lambda a, b: a * b,  # product of elements
        lambda _, b: b  # last element
    )

    print(acc.get_data())


def main():
    test_task1()
    test_task2()
    Runtime.test_duck_typing()


if __name__ == '__main__':
    main()
