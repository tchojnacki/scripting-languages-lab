class TextBuffer:
    '''
    Wrapper class around a string read from a file.
    '''

    def __init__(self):
        super().__init__()
        self.text = ''

    def read_from_file(self, path: str):
        '''
        Modifies the class by updating the text field to instead contain contents of the given file.
        '''

        with open(path, 'r', encoding='utf-8') as file:
            self.text = file.read()
