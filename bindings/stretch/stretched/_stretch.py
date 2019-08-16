from stretched import _bindings

# ========================================================================= #
# STRETCH                                                                   #
# ========================================================================= #


class Stretch:
    _PRIVATE_PTR: int = None

    def __init__(self):
        raise Exception('You should not be accessing or attempting to create an instance of this class.')

    @staticmethod
    def get_ptr():
        if Stretch._PRIVATE_PTR is None:
            Stretch._PRIVATE_PTR = _bindings.stretch_init()
        return Stretch._PRIVATE_PTR


# ========================================================================= #
# END                                                                       #
# ========================================================================= #
