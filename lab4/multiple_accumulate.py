from functools import reduce


class MultipleAccumulate:
    def __init__(self, data_list, *accumulate_functions):
        self.data_list = data_list
        self.accumulate_functions = accumulate_functions

    def get_data(self):
        def fun_name(fun, i):
            if fun.__name__ == '<lambda>':
                return f'lambda{i}'
            return fun.__name__

        return {
            fun_name(fun, i): reduce(fun, self.data_list)
            for i, fun in enumerate(self.accumulate_functions)
        }
