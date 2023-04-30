def decorate(fn):
    def wrapper(*args, **kwargs):
        print("----------------------------")
        ret = fn(*args, **kwargs)
        print("----------------------------")
        return ret

    return wrapper

