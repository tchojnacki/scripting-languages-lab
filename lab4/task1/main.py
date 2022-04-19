from viewer_creator import ViewerCreator


def main():
    creator = ViewerCreator()

    image_viewer = creator.create_viewer('test_files\\samoyed.jpg')
    image_viewer.view()

    text_viewer = creator.create_viewer('test_files\\tadeusz.txt')
    text_viewer.view()

    stats = text_viewer.get_data()
    print(f'lines: {stats.number_of_lines}')
    print(f'nonalpha: {stats.number_of_nonalpha}')
    print(f'words: {stats.number_of_words}')


if __name__ == '__main__':
    main()
