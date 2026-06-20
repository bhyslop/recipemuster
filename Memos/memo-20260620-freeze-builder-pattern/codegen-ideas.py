## Copyright Scale Invariant, Inc - All Rights Reserved
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> March 2019


import base64
import collections
import inspect
import os
import re
import sys

from typing import Dict, List, Union, Callable


GBU_PACKAGE_GBU     = 'GBU' # best package ever
GBU_PACKAGE_GOU     = 'GOU' # Package of the Generate Object Utility
GBU_PACKAGE_GCU     = 'GCU' # Package of the Generate C Code Utility
GBU_PACKAGE_RMU     = 'RMU' # Package of the Render Math Utility
GBU_PACKAGE_TAU     = 'TAU' # Package of the Type Annotation Utility

GBU_TERM_BUILDER     = 'Builder'  # Class that accumulates meaning through methods until it freezes for data model use
GBU_TERM_INITIALIZER = '__init__ function' # Name I keep using to refer to the python class constructor
GBU_TERM_OCONTEXT    = 'OContext'
GBU_TERM_TODO        = 'TODO'
GBU_TERM_XAN         = 'Xan' # ITCH_XanOroOcsFix: so far from okay name: lets go for gold i.e. ORO!  Whoops, there's OCS in the way

GBU_IDEA_OPAQUE      = 'opaque'
GBU_IDEA_FREEZE      = 'freeze'


GBU_UNDERSCORE     = '_'

# Someday maybe TERMize these, discerned with an AI.  Not all are proper though.
# 1. Callable
# 2. DeferredConstant
# 3. Exception
# 4. Freezable
# 5. GrepTag
# 6. IceContextManager
# 7. IceDictionary
# 8. IceList
# 9. Immutable
# 10. Locale
# 11. MultiFreezableContextManager
# 12. ObjectContextSet
# 13. OneNonnoneChecker
# 14. OneTrueChecker
# 15. Token
# 16. Trace
# 17. Various utility functions




# ITCH_LaterFastFix: memberize
def _gbu_freeze_trace(obj, note=""):
    if False:
        print(f"FREEZETRACE: {obj.__class__.__name__}: {note}")


class gbu_Exception(Exception):
    f"""
    The pattern of exception processing in this codebase is explicitly and unabashedly
    personal.  Each master module ('utility') has its own Exception implementation
    that is just like this one, no between-module derivation.  Then, each also has
    equivalent functions of gbux_ below for checking and tossing exceptions in a
    common way.  While the Exception classes are not derived from each other, the
    equivalent gbux_ functions do delegate.  Exceptions in this codebase are explicitly
    failures that must be resolved, not regular mechanisms of function.  This pattern
    makes it easy to generate a lot of them cheaply so that mainline code stays
    legible, but allows some good testing in critical areas.

    A brief note on the -isms of naming: I pretty much like the googletest postfixes
    that do NE, EQ, LE, LT, et cetera.  For the moment I'm not recreating all that
    here.  I don't need it yet, and I haven't found resolve with the separation of
    `NE` from `Now` for the postfix of immediate unconditional fail.  Someday perhaps!
    """

    def __init__(self, noteOrLambda):
        super().__init__(f"cnmexc: {noteOrLambda() if callable(noteOrLambda) else noteOrLambda}")

# ITCH_ExceptionLambaize == z See what performance enhancements from lambda-izing- real?

# ITCH_ExceptionPassRV == z Maybe make _dieIf and _dieUnless pass a return value if no exception
#    Looking at some _greptag functions, I can make the death even shorter if I can allow
#    the conditional exception checkers pass a happy return value through.  Hmm, is it enough to
#    return None from the gbux_die{If,Unless} functions?  I should be able to do...
#
#      return _dieIf(cond, "text") or myGoodRetVal
#
#    Lets try it sometime first, before polluting all signatures with extra arg

def gbux_dieNow(exceptionClass, noteOrLambda):
    f"""Unconditional exception creation.  Do not use directly: use a module _dieNow function instead."""
    raise exceptionClass(noteOrLambda)

# ITCH_ExceptionPassRV: here
def gbux_dieIf(condition, exceptionClass, noteOrLambda):
    f"""Exception only if a condition is true.  Do not use directly: use a module _dieIf function instead."""
    if condition:
        raise exceptionClass(noteOrLambda)
    return None

# ITCH_ExceptionPassRV: here
def gbux_dieUnless(condition, exceptionClass, noteOrLambda):
    f"""Exception only if a condition is false.  Do not use directly: use a module _dieIf function instead."""
    if not condition:
        raise exceptionClass(noteOrLambda)
    return None

def _dieNow(               noteOrLambda): return gbux_dieNow(               gbu_Exception, noteOrLambda)
def _dieIf(     condition, noteOrLambda): return gbux_dieIf(     condition, gbu_Exception, noteOrLambda)
def _dieUnless( condition, noteOrLambda): return gbux_dieUnless( condition, gbu_Exception, noteOrLambda)
# OUCH GBU: raise -> _dieIf 


# ITCH_gbuFreezeDecorator == y Shift freezing behaviors to gbu_once like decorators
#                            Maybe @fzc_frozen, @fzc_unfrozen, @fzc_freezing which all act on 
#                            things derived from this class for assuring the appropriate state.
#                            I'll observe that several operations have 'assert if frozen' or
#                            'assert if not frozen' in them: this behavior can be pulled out
#                            to the decorator.  Challengingly, sometimes I even allow an op
#                            to carry out the freeze itself.  I am not sure what I want but
#                            I like the idea of carrying this to the decorator.  Before
#                            attempting this, consider also the impact of multiphase freezes
#                            described in the docstring below.  For now I am expecting to
#                            heavily destress the freeze system by making a lot more use of
#                            immutables and 'database normalization' in the management of very
#                            sophisticated data models.
class gbu_Freezable:
    f"""
    Base class for objects that live for some part of their lifecycle as mutable, then
    at some point they {GBU_IDEA_FREEZE}; thereafter, that object is immutable.  The typical
    case for this pattern is in data modelling where during data model expansion, it
    is useful to use the object as a 'builder' to weave all relationships necessary
    between it and others.  As soon as information is extracted following building, the
    object must not be allowed to change or else subsequent extractions may be
    inconsistent.

    For best use of this pattern, the 'freeze' event should be triggered any time that 
    something outside of the object attempts to see inside of it.  As soon as something 
    outside of the object starts to rely on its intimate details, it is really a good 
    idea to make absolutely certain that the intimate details will never change thereafter.

    The pattern described above has one specific exception that has been proven innocuous
    in a prior large project: if an attribute is assigned only in the __init__ function,
    then peeking at that attribute without triggering an object freeze is ok.


    DERIVED CLASS TEMPLATE:

    def __init__(self, yyy):
        # FREEZABLE: base initializer
        super().__init__()
        # FREEZABLE: Attributes
        # FREEZABLE: Builder mutable
        # FREEZABLE: mutable members for caching post-freeze derivations
        # FREEZABLE: end

    def _fzc_onFreeze(self): self.fzc_todo("Implement me")


    SOMEDAY add services that assure several things about member data and method
    such as prefixes that cause auto-freezing and ones that assert already frozen.
    Can for members even assert if a member holds a freezable which is not connected
    to this objects freeze system.

    SOMEDAY consider a whole concept map update but use phases of solids, i.e.
    'plasma', 'gas', 'liquid', 'solid' to indicate the level of freezing.

    FREEZE THAT DOESN'T PROPAGATE:  In some future refinement of the freezing model
    laid out here, I'm reasonably sure that I don't need the freezing of the set of
    members in a collection does _not_ need to immediately propagate into the members
    themselves.  It has come up enough times that I want 'leaves' to stay unfrozen
    even when the nester freezes.  This is okay unless in the peculiar instance of
    the nester needing to peek at 'leaves' during freezing, in which case of course
    they freeze.  But not demanded by the outer data structure itself.
        ITCH_FreezeModelUpgrades == z Freeze Model Improvements

    RELAXATION BY OCONTEXT OR OTHER LOOKUPS: I caught a glimmer of a pattern today
    where it would make great sense to have an unfrozen object have collections
    that are retrievable without triggering a freeze.  The pattern would be, you
    are allowed to presume existence of a member and extract it as long as you are
    not asking if it exists.  I.e. you aren't going to learn something about the
    outer that can change in the future- if you are asking, then you know.

    ITCH_FreezeLoopDetect == z Fatal on detection of freeze loop
       It has happened a few times where a freeze of one object ends up attempting
       to freeze itself down the stack: consider early detect and fail on this.
    """

    def __init__(self):
        self._fzc_isFrozen        = False
        self._gbuf_preFreezeList  = []
        self._gbuf_postFreezeList = []
        _gbu_freeze_trace(self, "creation")

    def _fzc_onFreeze(self):
        """Subclasses override this to trigger actions on initial freeze of an object.  It is
        never explicitly called by users: they call the _freeze method."""
        pass

    def _fzc_onFreezeBeta(self):
        """Sometimes, a whole class inheritance scheme has freezing at different levels.  This
        method is always called during freeze and is overridable accordingly."""
        pass

    def _fzc_onFreezeGamma(self):
        """Sometimes, a whole class inheritance scheme has freezing at different levels.  This
        method is always called during freeze and is overridable accordingly."""
        pass

    def __setattr__(self, name, value):
        """Someday it would be great to assure uniqueness and perhaps even acronymic coherence
        of all members of each class.  In other words, assure that each member of an object of
        type xyz_Class has a prefix which is xyzc_ or XYZC_ or possibly _xyzc_ such that global
        member uniqueness is protected.

        I'm less sure of this: these could also be checked on a corresponding __getattr__ to make
        sure that the type is valid.  Though this might be redundant with the natural language
        itself.

        Maybe a better idea: make this all be part of an even baser class than Freezable or
        Immutable, then add a required constructor parameter which locks in the prefix rule.

        This would pave the way for breaking out and commonalizing the module prefix, so renames
        are painful but complete.  To clarify, what if the pattern is:

            class xyz_MyClass(gbu_Freezable):
                def __init__(self, blah):
                    super().__init__(XYZ_PREFIX_STRING, "mc")
                    self.XYZMC_THING = object() # works
                    self.XYZXX_ITEM  = object() # THROWS
                    self.TBDMC_ITEM  = object() # THROWS

        One other wrinkle to make a ruling on: I discovered in one project that I liked having
        some explicit global prefixes to indicate calls that are available from a certain use
        perspective.  With extreme cleverness, this might be made super cool such that such a
        method which has a special common access pattern usage could be able to compile a list
        of all such usage group uses for the purpose of providing a unified documentation view
        to the user.  With Super Coolness Mode, any such method which delegates to a class
        constructor via *args, **kwargs pattern could reach in and pull that documentation out.
        I _think_ its possible :)

        NOTE_WITH_DISAPPOINTMENT: I tried to do the change above as a part of something else,
        and I've discovered that I really need to work at this because the order in which
        things happen when defining a __setattr__ are not intuitive.  Python is amazing for
        its powers, but beware constructing sub languages in it!  Come back one day, but I'm
        hoping I'll remember that it is not just a drop in.

        Also, beware the differences across generations of python- there's now a sophisticated
        set of alternatives and considerations given the froth of evolution.  The rules for
        inheritance as they apply to revising __setattr__ are weird for sure.  Let it go!
        """
        isInitCriteria = sys._getframe(1).f_code.co_name == '__init__'
        _gbu_freeze_trace(self, "attempting member add")
        _dieIf(not isInitCriteria and self._fzc_isFrozen,
               f"Cannot add members to a frozen object of type {type(self).__name__}")
        self.__dict__[name] = value

    def fzc_prefreeze(self, freezable):
        """Add to a list of freezables kept by this object which are automatically frozen
        _just before_ the object's own freezable defined function.  Defined for chaining."""
        _gbu_freeze_trace(self, "register for prefreeze")
        _dieIf(self._fzc_isFrozen, "Not allowed without reason")
        self.fzc_raiseIfFrozen()._gbuf_preFreezeList.append(freezable)
        return freezable

    def fzc_autofreeze(self, freezable):
        """Legacy name."""
        _gbu_freeze_trace(self, "register for autofreeze")
        return self.fzc_prefreeze(freezable)

    # ITCH_LaterFastFix: fzc_dict rename
    def fzc_iceDict(self,
                    keyClassOrNone=None,
                    valueClassOrNone=None,
                    enableFullDupeInsertion=True, # Determining python equality not appropriate for all value types
                    ):
        return self.fzc_autofreeze(gbu_IceDictionary(keyClassOrNone=keyClassOrNone,
                                                     valueClassOrNone=valueClassOrNone,
                                                     enableFullDupeInsertion=enableFullDupeInsertion))

    # ITCH_LaterFastFix: fzc_list rename
    def fzc_iceList(self, *args, **kwargs):
        return self.fzc_autofreeze(gbu_IceList(*args, **kwargs))

    def fzc_postfreeze(self, freezable):
        """Add to a list of freezables kept by this object which are automatically frozen
        _following_ the object's own freezable defined function.  Defined for chaining."""
        _gbu_freeze_trace(self, "register for postfreeze")
        _dieIf(self._fzc_isFrozen, "Not allowed without reason")
        self.fzc_raiseIfFrozen()._gbuf_postFreezeList.append(freezable)
        return freezable

    def fzc_freeze(self):
        """Any subclass method can call this to trigger a freeze.  Note that this will
        trigger the internal, one-time freezing behavior if the object is not frozen.  If
        the object is frozen, this call has no effect."""
        if not self._fzc_isFrozen:
            _gbu_freeze_trace(self, "starting main freeze")
            for freezable in self._gbuf_preFreezeList:
                freezable.fzc_freeze()
            self._fzc_onFreezeGamma()
            self._fzc_onFreezeBeta()
            self._fzc_onFreeze()
            for freezable in self._gbuf_postFreezeList:
                freezable.fzc_freeze()
            self._fzc_isFrozen = True
        else:
            _gbu_freeze_trace(self, "object already frozen")
        return self

    def fzc_raiseIfFrozen(self):
        """Raise an exception if the class has already been frozen.  This should be checked
        every time that a function useful only during the 'builder' phase of the object's 
        lifecycle."""

        if not self._fzc_isFrozen: return self
        _dieNow(f"Frozen {self.__class__.__name__} found where unfrozen needed")

    def fzc_raiseUnlessFrozen(self):
        """Raise an exception unless the class has already been frozen.  This is only useful
        when doing diagnostics that expect class already frozen.  Otherwise, just freeze."""

        _gbu_freeze_trace(self, "about to test frozen")
        if self._fzc_isFrozen: return self
        _dieNow(f"Unfrozen object of type {type(self).__name__} found where frozen expected")

    def fzc_todo(self, comment=None): 
        """A utility to mark that some aspect of the class needs more implementation."""
        _dieNow(f"MUST FINISH {comment}")

    def fzc_getCM(self) -> 'gbu_IceContextManager':
        """Routine that lets you create a context manager which freezes the object
        at the end.  That then allows statements like:

        with obj.fzc_getCM() as x:
            x.doUnfrozenStuff()

        # Then here, obj is frozen

        SOMEDAY consider name change  ->  _with
        """
        return gbu_IceContextManager(self)


def gbu_prefreeze(method):
    def _gbu_prefreezeDecorator(self, *args, **kwargs):
        self.fzc_raiseIfFrozen()
        result = method(self, *args, **kwargs)
        self.fzc_raiseIfFrozen()
        return result
    return _gbu_prefreezeDecorator


def gbu_postfreeze(method):
    def _gbu_postfreezeDecorator(self : gbu_Freezable, *args, **kwargs):
        # I experimented with checking for frozenness instead of freezing, but not obvious fails
        self.fzc_freeze()
        result = method(self, *args, **kwargs)
        return result
    return _gbu_postfreezeDecorator


class gbu_IceContextManager:
    """A shorthand way to use a freezable for a short period such that it gets frozen
    upon exit of the context manager."""

    # SOMEDAY remove the finalizableList because it leads to side effects
    def __init__(self, freezable : gbu_Freezable, finalizableList = []):
        if isinstance(freezable, gbu_Freezable):
            # At the time of this writing, I can't think of a reason why
            #  this should be allowed.
            freezable.fzc_raiseIfFrozen()
        self.GBUICM_FREEZABLE = freezable
        self._gbuicm_onExitFinalizableTuple = tuple(finalizableList)

    def __enter__(self):
        return self.GBUICM_FREEZABLE.fzc_raiseIfFrozen()

    def __exit__(self, type, value, traceback):
        self.GBUICM_FREEZABLE.fzc_freeze()
        for finalizable in self._gbuicm_onExitFinalizableTuple:
            finalizable()



class gbu_MultiFreezableContextManager:
    """Expansion on previous."""

    # SOMEDAY remove the finalizableList because it leads to side effects
    def __init__(self,
                 freezableList : List[gbu_Freezable],
                 finalizableList = []):
        for freezable in freezableList: 
            if isinstance(freezable, gbu_Freezable):
                # At the time of this writing, I can't think of a reason why
                #  this should allow a frozen item to go in the list of things
                #  that need to be frozen at the end of the context block, but
                #  at best this just prevents common case surprise.  This could
                #  be bent without issue.
                freezable.fzc_raiseIfFrozen()
        self._gbumfcm_onExitFreezables    = tuple(freezableList)
        self._gbumfcm_onExitFinalizeables = tuple(finalizableList)

    def __enter__(self):
        return self._gbumfcm_onExitFreezables

    def __exit__(self, type, value, traceback):
        for freezable in self._gbumfcm_onExitFreezables:
            if isinstance(freezable, gbu_Freezable): freezable.fzc_freeze()
        for finalizable in self._gbumfcm_onExitFinalizeables:
            finalizable()


class gbu_DeferredConstant:
    """I came across a strange freeze loop where I need immutable members that don't
    set until they are first inspected.  This class produces yet another variant of
    a freeze pattern.

    https://stackoverflow.com/questions/8062161/can-i-dynamically-convert-an-instance-of-one-class-to-another
    https://stackoverflow.com/questions/2153295/python-object-conversion
    https://stackoverflow.com/questions/1638229/python-class-with-integer-emulation
    https://www.python.org/download/releases/2.2.3/descrintro/#__new__
    https://realpython.com/inherit-python-str/
    https://stackoverflow.com/questions/64916860/how-to-assign-to-a-base-class-object-when-inheriting-from-builtin-objects

    """

    def __init__(self, forceClass, lambdaGenerator):
        self.GBUDC_CLASS    = forceClass
        self.__gbudc_lambda = gbu_assureLambda(lambdaGenerator)
        self.__dbudc_const  = None

    def gbudc_value(self):
        if self.__gbudc_lambda is not None:
            lambdaReturn = self.__gbudc_lambda()
            self.__dbudc_const = gbu_needType(lambdaReturn, self.GBUDC_CLASS)
            self.__gbudc_lambda = None
        return self.__dbudc_const


# ITCH_gbuBetterTests: add test case showing that multiple layers of derivation
#      initializer works with the immutable constraints: can still add 'attributes'
#      in derived initializer, both levels later available, but not modifiable and
#      certainly can't still create.
class gbu_Immutable:
    """Classes that derive from this class do not allow setting of attributes except from the
    __init__ call.  At the time of this writing, this class won't prevent members which are
    structured or collections from having their objects modified.

    ITCH_BetterImmutability == z Improve immutability features
    ITCH_BetterImmutability: I think I can make this class hashable cheaply with some subtle
        engineering.  An AI tells me that making something hashable is as simple as properly
        defining __hash__ and __eq__: if true (and best to find primary documents for this
        before embarking) then I should be able to fold these things into the lifecycle of
        this object such that they are computed at __init__ and simply returned.  Hmm, could
        such a treatment also work with freezables?

    DERIVED CLASS TEMPLATE:

    def __init__(self, yyy):
        # IMMUTABLE: base initializer
        super().__init__()
        # IMMUTABLE: Attributes
    """

    def __init__(self):
        self.GBUI_INITIALIZER_CALLED = True

    # See http://code.activestate.com/recipes/252158-how-to-freeze-python-classes/
    # See https://python-reference.readthedocs.io/en/latest/docs/dunderattr/setattr.html

    def __setattr__(self, name, value):
        if name != "GBUI_INITIALIZER_CALLED":
            _dieIf(sys._getframe(1).f_code.co_name != '__init__',
                   f"Immutable object for class {self.__class__.__name__}")
            _dieUnless(self.GBUI_INITIALIZER_CALLED, "See comment above") # A class deriving from this failed to call super().__init__, so this is not defined
        self.__dict__[name] = value

    def __getattribute__(self, name):
        rv = object.__getattribute__(self, name)

        # Nesting badly will throw this off
        while isinstance(rv, gbu_DeferredConstant):
            rv = rv.gbudc_value()
            self.__dict__[name] = rv
        return rv

    def fzc_dict(self,
                 keyClassOrNone=None,
                 valueClassOrNone=None,
                 enableFullDupeInsertion=True, # Determining python equality not appropriate for all value types
                 ):
        return gbu_IceDictionary(keyClassOrNone=keyClassOrNone,
                                 valueClassOrNone=valueClassOrNone,
                                 enableFullDupeInsertion=enableFullDupeInsertion)

    def fzc_list(self, *args, **kwargs): # ITCH_LaterFastFix: fzc_list rename
        return gbu_IceList(*args, **kwargs)


class gbu_Token(gbu_Immutable):
    """Simple class used to represent a string that is a valid identifier in all languages you
    might want to generate code for.  C99 token rules are the most restrictive considered at this
    time."""

    # SOMEDAY assess global token definitions differently than local, specifically with
    #         respect to leading _  characters (not allowed globally but ok locally)
    _gmut_checkerCxx = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
    _gmut_checkerCpp = re.compile(r"^((?:[a-zA-Z_][a-zA-Z0-9_]*::)*[a-zA-Z_][a-zA-Z0-9_]*)$")

    @staticmethod
    def gbut_raiseUnlessValid(string, cppOkay=False):
        checker = gbu_Token._gmut_checkerCpp if cppOkay else gbu_Token._gmut_checkerCxx
        _dieUnless(checker.match(string), f"Invalid token {string}")
        return string

    @staticmethod
    def gbut_isValid(string, cppOkay=False):
        checker = gbu_Token._gmut_checkerCpp if cppOkay else gbu_Token._gmut_checkerCxx
        return checker.match(string)

    def __init__(self, name):
        super().__init__()
        self.gbut_raiseUnlessValid(name)
        self.GBUT_NAME = name

    def __str__(self): 
        return "{{{%s: %s}}}" % (self.__class__, self.GBUT_NAME)


def gbu_iterable(item):
    try:
        iter(item)
        return True
    except TypeError:
        return False

# ITCH_XanOroOcsFix == z Nuke OCS in favor of ORO currently drafted Xan
# 
# ITCH_ExceptionX == z use letter 'x' for Exception i.e. gbux_ because of its power!
# ITCH_ExceptionX: depends on solving ITCH_XanOroOcsFix
#      then, what do I name this?  Aha, MAP!  Well, the term map() is actually a python
#      primitive.  Claude suggests 'zag'.  ChatGPT additionally suggests 'vex' and 'zest'.
#      It makes sense that this is functioning like the c++ multimap idea, though this is
#      a little bit more specific.
class gbu_Xan(gbu_Freezable):
    f"""A {GBU_TERM_XAN} is a repository for stashing and recovering arbitrary
    objects using arbitrary keys supporting code generation.  This is an attempted
    replacement for {GBU_TERM_OCONTEXT} used in a prior generation. This follows
    the principle of 'freezing' where the library is mutable until information is
    retrieved from it at which time it becomes immutable.  

    Reference objects are often named {GBU_IDEA_OPAQUE}.

    This is different from the python dictionary in a few ways.
    * Key objects need not be hashable: this uses 'id' instead.

    ITCH_XanOroOcsFix: revise below after rename
    The name is chosen to be short and easily pronouncable since the
    prior {GBU_TERM_OCONTEXT} experiment shows these things may be
    quite ubiquitous.

    Note briefly: python _will_ reuse unique object identifiers it seems, if the 
    prior one has fully gone out of scope.  See transcript below for how I triggered
    this.  This design is not vulnerable to that since it retains all candidate
    objects in the backwards dictionary, thus assuring their ids stay present.
            >>> id("DUDE")
            140382469441968
            >>> id("DUDER")
            140382469441968
            >>> id("DUDE")
            140382469441968
            >>>

    """

    def __init__(self):
        super().__init__()
        self.__working_r2Item = {}
        self.__working_i2referenceSet = {}

    # ITCH_XanOroOcsFix: Old comment- meaningful?  
    #     must add a variant of below that allows iterable references, but not arbitrary depths

    @gbu_prefreeze
    def gbux_link(self, item, reference):

        # Until really necessary, disallow Nones
        _dieUnless(item      is not None, "Item cannot be None")
        _dieUnless(reference is not None, "Reference cannot be None")

        i = id(item)
        r = id(reference)

        # First, the item itself can be its own reference.  It is not illegal for duplicate
        #      request attempt, though obtuse if the item somehow refers to a different item.
        if i in self.__working_r2Item:
            existingItemId = id(self.__working_r2Item[i])
            _dieUnless(existingItemId == i, f"Item {i} already mapped to different item")
        else:
            self.__working_r2Item[i] = item

        # Now add the reference allow duplicates so we can use above op in below
        beforeItem = self.__working_r2Item.get(r)
        if beforeItem is not None:
            _dieUnless(beforeItem == item, f"Reference {r} already mapped to different item")
        else:
            self.__working_r2Item[r] = item

        # Finally, recompose the reverse links
        refs = self.__working_i2referenceSet.get(i, set())
        refs.add(item)
        refs.add(reference)
        self.__working_i2referenceSet[i] = refs

    @gbu_prefreeze
    def gbux_grandfather(self, item, xan, xanItemOrReference):
        # ITCH_XanOroOcsFix: do a good docstring here, relate to _link right.  Or unify?
        xan.fzc_freeze()
        _dieIf(item is None, "Item cannot be None")
        r = id(xanItemOrReference)
        xanItem = xan.__frozen_r2Item.get(r, None)
        _dieIf(xanItem is None, f"Reference {r} not found in parent {GBU_TERM_XAN}")

        for reference in xan.__frozen_i2referenceSet[id(xanItem)]:
            self.gbux_link(item, reference)
        
    def _fzc_onFreeze(self):
        self.__frozen_i2referenceSet = {i: frozenset(s) for i, s in self.__working_i2referenceSet.items()}
        del                                                         self.__working_i2referenceSet
        self.__frozen_r2Item = self.__working_r2Item
        del                    self.__working_r2Item

    @gbu_postfreeze
    def gbux_item(self, itemOrReference):
        r = id(itemOrReference)
        _dieIf(r not in self.__frozen_r2Item, f"Reference {r} not found")
        return                self.__frozen_r2Item[r]
    

class gbu_ObjectContextSet(gbu_Immutable):
    """It is starting to be clear that sometimes I want many ways to re-access something
    that goes into one of the codegen collections.  This helps create such lookups.  This
    is somewhat experimental today: will this just be a mess?  To be determined...

    ITCH_OContextSetScrub == z reunify object context patterns
    ITCH_OContextSetScrub: Does Xan class make this unneeded?
    """

    def __init__(self, *items):
        super().__init__()

        flattened = []

        while len(items):
            candidates = items
            items = []
            for item in candidates:
                if False: pass
                elif item is None:
                    pass
                elif isinstance(item, str):
                    flattened.append(item)
                elif gbu_iterable(item):
                    items = items + list(item)
                elif isinstance(item, gbu_ObjectContextSet):
                    flattened = flattened + list(item.GBUOCS_ITEMS) # Add members, not collection
                else:
                    flattened.append(item)

        self.GBUOCS_ITEMS = tuple(set(flattened))

GBU_OCS_EMPTY = gbu_ObjectContextSet()

class gbu_IceList(gbu_Freezable):
    """A support class that acts like Python native lists except that as soon as 
    any member is queried, all freezable elements are frozen and thereafter, 
    any attempt to change the contents of the list results in exception."""

    def __init__(self, elementClass=None):
        super().__init__()
        self.GBUIL_ELEMENT_CLASS = elementClass
        self._gbuil_innerList    = []

    def append(self, item): _dieNow("Not supported at the moment, but could change")

    def gbuil_append(self, item):
        self.fzc_raiseIfFrozen()
        _dieIf(self.GBUIL_ELEMENT_CLASS and not isinstance(item, self.GBUIL_ELEMENT_CLASS),
               f"Expected type {self.GBUIL_ELEMENT_CLASS} but saw {item.__class__}")
        self._gbuil_innerList.append(item)
        return item

    def gbuil_raiseIfLongerThan(self, otherIceList : 'gbu_IceList', text):
        """Use this for integrity checks between data structures that do not cause freezing."""
        _dieIf(len(self._gbuil_innerList) > len(otherIceList._gbuil_innerList), f"length violation: {text}")

    def _fzc_onFreeze(self):
        # I'm not 100% certain it is wise to freeze all elements when just the collection freezes.
        #  Let me know if this bites...
        for elem in self._gbuil_innerList:
            if isinstance(elem, gbu_Freezable): elem.fzc_freeze()

    def gbuil_getOnly(self):
        elements = len(self.fzc_freeze()._gbuil_innerList)
        _dieIf(elements != 1, f"Expected single element but found {elements}")
        return self._gbuil_innerList[0]

    def __setitem__(self, idx, value): 
        prevItem = self._gbuil_innerList[idx]
        _dieIf(prevItem is not None, f"Item already assigned {idx=} {value=}")
        self.fzc_raiseIfFrozen()._gbuil_innerList[idx] = value

    def __getitem__(self, idx): return self.fzc_freeze()._gbuil_innerList[idx]

    def __len__(self): return self.fzc_freeze()._gbuil_innerList.__len__()

    def __iter__(self): yield from self.fzc_freeze()._gbuil_innerList

    def gbuil_values(self): return [x for x in self]

def gbu_makeFrozenList(elementClass, elementList):
    rv = gbu_IceList(elementClass)
    for element in elementList:
        rv.gbuil_append(element)
    return rv.fzc_freeze()

class gbu_IceDictionary(gbu_Freezable, collections.abc.MutableMapping):
    f"""This is an order preserving key/value collection that will raise an
    Exception if a duplicate key is being reinserted with a different value.

    I'm going to try and avoid ambiguating whether such a collection is a
    standard Python dictionary by keeping the prefix discipline present.

    Good help from --> https://stackoverflow.com/questions/3387691/how-to-perfectly-override-a-dict

    ITCH_gbuFrozenKeysOnly == z Ability to make an immutable iterable set of unfrozen things
       From a minidrama, I realized that sometimes I want a new peer to `IceDictionary` which
       differentiates the freezing of the set of 'key' members (and expressed order) without
       requiring values to freeze until later.  This showed up in Fief as well as Shape where
       I want to use members of an object reflecting concrete selection of Cells before I can
       fully freeze the Cells.  This notional new thing probably needs list semantics, i.e.
       remembers order of entry, but functions like a set, so its not really a dictionary.
       Well, I've wanted it twice...  Safe to temporarily just use a dundered OrderedDict
       during building and then on freeze nuke the builder object while creating the fully
       frozen version.  Better yet, keep both at same time, simply nuke the one used during
       building at freeze.

    ITCH_gbuFrozenKeysOnly: LATER ADD, third encounter.  There was some wicked
       cool stuff I was doing with inline dictionaries that got foiled.  Alas!  Come back 
       later, and on attack, lets actually start with unit tests proving that the de-facto
       freezing of the set of elements/ keys need not propagate to the freezing of the 
       actual values, though that should still happen when the object is formally frozen.

    ITCH_gbuFrozenKeysOnly: LATER LATER ADD.  I'm cauterizing o-u-c-h statements and am
       suddenly unsure under which terms that I want keys to freeze.  Should they be actual
       object ids like the {GBU_TERM_XAN} organizer instead?  A freezing event may only be
       triggered on a lookup, and that may not actually properly need the keys to freeze,
       just trigger the container to stop accepting inputs.
    """

    def __init__(self,
                 keyClassOrNone=None,
                 valueClassOrNone=None,
                 enableFullDupeInsertion=True, # Determining python equality not appropriate for all value types
                 ):
        super().__init__()

        self.GBUID_KEY_CLASS   = keyClassOrNone
        self.GBUID_VALUE_CLASS = valueClassOrNone

        self._gbuid_enableFullDupe = enableFullDupeInsertion
        self._gbuid_dict           = collections.OrderedDict()

    def gbuid_update(self, **kwargs):
        self.fzc_raiseIfFrozen()
        self.update(dict(**kwargs))

    def _fzc_onFreeze(self):
        # I'm not 100% certain it is wise to freeze all elements when just the collection freezes.
        #  Reconsider if this causes a problem...
        for key, value in self._gbuid_dict.items():
            # ITCH_gbuFrozenKeysOnly: above, I now think I don't want to add 
            #    the following, though once I did:
            #        if isinstance(key,   gbu_Freezable): key.fzc_freeze()
            if isinstance(value, gbu_Freezable): value.fzc_freeze()

    def gbuid_insert(self, key, value):
        # ITCH_gbuFrozenKeysOnly: WHY IS THIS BAD? -> self.fzc_raiseIfFrozen()
        self[key] = value
        return value

    def gbuid_keys(self): return self.fzc_freeze()._gbuid_dict.keys()

    # It actually seems that MutableMapping provides functioning .values statement, wow. Keep for now
    # SOMEDAY get very clear that if .values works, whether it needs a forced freeze
    def gbuid_values(self): return tuple(self.fzc_freeze()._gbuid_dict.values())

    def gbuid_valuesSortedByKey(self):
        self.fzc_freeze()
        return [self._gbuid_dict[key] for key in sorted(self._gbuid_dict.keys())]

    def __getitem__(self, key):
        self.fzc_freeze()
        # ITCH_LaterEasierErrors: why did this fail?  == z Improve error checking when lots of run coverage
        #     Aha, the problem seems to be that when you use the classic dictionary  ->  .get(key, default)
        #     ... you end up triggering the below logic, wrongly.

        # if key not in self._gbuid_dict:
        #     values = self._gbuid_dict.values()
        #     retstring = [f"Could not find key:{key} amongst {len(values)} candidates:"] +\
        #                 [f"  {value}" for value in values]
        #     raise Exception('\n'.join(retstring))
        return self._gbuid_dict[key]

    def __setitem__(self, key, value):
        self.fzc_raiseIfFrozen()
        _dieIf(self.GBUID_KEY_CLASS and not isinstance(key, self.GBUID_KEY_CLASS),
               f"Bad key class: expected {self.GBUID_KEY_CLASS} but saw {key.__class__}")

        _dieIf(self.GBUID_VALUE_CLASS and not isinstance(value, self.GBUID_VALUE_CLASS),
               f"Bad value class: expected {self.GBUID_VALUE_CLASS} but saw {value.__class__}")

        if key in self._gbuid_dict: 
            existingValue = self._gbuid_dict[key]
            if self._gbuid_enableFullDupe:
                _dieIf(existingValue != value,
                       f"At key:{key if isinstance(key, str) else gbu_id(key)} " +
                       f"tried to store value:{gbu_id(value)} but existing:{gbu_id(existingValue)} was already there")
                # Allowable to reinsert same thing otherwise
            else:
                _dieNow(f"{key=} already present in dictionary with {existingValue=} while trying to insert {value=}")

        self.fzc_raiseIfFrozen()._gbuid_dict[key] = value

    def __delitem__(self): _dieNow("I don't know why this should be allowed")

    def __iter__(self): return iter(self.fzc_freeze()._gbuid_dict)

    def __len__(self): return len(self.fzc_freeze()._gbuid_dict)

    def __keytransform__(self, key): 
        # I don't understand this; taking it from boilerplate
        self.fzc_freeze()
        return key

# ITCH_ScrubVscodeNags == z reduce nags produced by Visual Studio Code
#    Claude.ai generated a wonderful clean up possibility for nags from indefinite
#    signatures.  When I'm back in that world, lets see if the following indeed is
#    better:
#  
#        from typing import Any, Dict, Type, TypeVar, Union
#        
#        KT = TypeVar("KT")
#        VT = TypeVar("VT")
#        
#        def gbu_makeFrozenDictionary(
#            keyClassOrNone: Union[Type[KT], None] = None,
#            valueClassOrNone: Union[Type[VT], None] = None,
#            *dictionaries: Dict[Any, Any]
#        ) -> Dict[KT, VT]:
#            rv: Dict[KT, VT] = gbu_IceDictionary(keyClassOrNone, valueClassOrNone)
#        
#            for dictionary in dictionaries:
#                if dictionary is not None:
#                    for key, value in dictionary.items():
#                        rv.gbuid_insert(key, value)
#        
#            return rv.fzc_freeze()
# ITCH_LaterFastFix: I really want a shorter name for this highly ubiquitous, very line-lengthen-y
#       utility really, gbu_dict is not too short I think.  gbu_fdict? gbu_iceDict? gbu_makeIceDict?
def gbu_makeFrozenDictionary(
        keyClassOrNone=None,
        valueClassOrNone=None,
        *dictionaries):
    rv = gbu_IceDictionary(keyClassOrNone, valueClassOrNone)
    for dictionary in dictionaries:
        if dictionary is not None:
            for key, value in dictionary.items():
                rv.gbuid_insert(key, value)
    return rv.fzc_freeze()

# ITCH_ScrubVscodeNags: probably same thing here as above.  Adding a type check
#     is better I think.  Dare I have options for None?  Probably only on actual need.
def gbu_makeFrozenTuple(items):  return tuple(z.fzc_freeze() for z in items)

def gbu_makeGeneralTuple(items, classOrNone=None):
    rv = tuple() if items is None else tuple(i for i in items)
    if classOrNone is not None:
        for item in rv:
            _dieUnless(isinstance(item, classOrNone), "Wrong type found in tuple")
    return rv

class gbu_Locale:
    """This retains a whole stack trace in an object such that it can be easily printed
    later- just name/lineno/file of functions, not their parameters.  Beware it prints
    in opposite of stack dump order.  Beware that taking a snapshot of the state of the 
    stack takes a long time, so these ought to be sparingly used.

    A very common use case is when a IceDictionary invariant is violated, i.e. we try
    and assign the same key twice: this can be debugged by keeping a locale every time
    something is inserted, printing it, and then later figuring out why you went down
    the same path again.
    """

    def __init__(self, stackTopDelta=1):
        locale = []
        recordList = inspect.stack()[stackTopDelta:]
        for record in recordList:
            frame = record[0]
            info = inspect.getframeinfo(frame)
            entry = f"{info.filename}({info.lineno}): function {info.function}"
            candySplit = entry.split("/") # Skip subdirectories, until a reason not to
            entry = candySplit[-1]
            locale.append(entry)
        self.GBUL_UPSTACK = tuple(locale)

    def gbul_matchesNLevels(self, other, n):
        for i in range(n):
            alpha = self.GBUL_UPSTACK[i]
            beta  = other.GBUL_UPSTACK[i]
            match = alpha == beta
            if not match: return False
        return True

    def __str__(self):
        return "\n".join(reversed(self.GBUL_UPSTACK))


def gbu_id(obj):
    """create a less hostile string for discerning an object instance."""
    pure = id(obj)
    bytes = pure.to_bytes((pure.bit_length() + 7) // 8, byteorder="big")
    encoded = base64.b64encode(bytes)
    asString = encoded.decode()
    return f"<{type(obj).__name__}:{asString}>"


import inspect
import os

def gbu_trace(message, onlyIf=True, showLocale=False, frameNudge=0):
    """
    Prints a trace message along with the filename and line number of the caller.

    Args:
        message (str or callable): The message to be printed.
        onlyIf (bool, optional): If True, the message will be printed. Defaults to True.
        showLocale (bool, optional): If True, the current locale will be printed. Defaults to False.
        frameNudge (int, optional): The number of frames to skip when determining the caller. Defaults to 0.
    """

    if not onlyIf: return

    if callable(message): message = message()

    caller = inspect.getframeinfo(inspect.stack(context=1 + frameNudge)[1 + frameNudge][0])
    _, filename = os.path.split(caller.filename)
    print("%s(%d): %s" % (filename, caller.lineno, message))

    if showLocale:
        string = str(gbu_Locale())
        for line in string.split("\n"):
            print(f"   {line}")
        print("")
        print("")


class gbu_GrepTag(gbu_Immutable):
    """A greptag is a string formatted in a precise way which associates a section of
    code used to generate something with the code that is generated.  This helps
    the engineer easily jump between the two locations.  As such, in the generator,
    it is important that these strings be not programmatically constructed.  Rather,
    they should be immediates. 

    This implementation presumes that a greptag looks like the following:

       :tdugt:

    Note leading and trailing colons, and the initials that correspond to (this)
    module.

    SOMEDAY make this class look at its stack trace and determine whether a greptag
    string is allowed or not based on precise match.  At the time of this writing,
    I have to store greptag strings in class variables so they can be reused when
    a particular function signature is multiply generated.

    ITCH_GreptagGluePlusLocale == z Concatenate multiple greptags also with decl locale
      I find that two greptags sometimes might be the best choice for getting back to
      the code that generates a region.  And sometimes, just sometimes, being able to
      swim back out to an Accord locale would be great.  How do this without massive
      lags in comp time?  Aha, the whole feature including the greptags themselves needs
      to be disableable for production and code delivery builds.
    """

    ENABLED = True # Disabling nearly halves the runtime...  ITCH_GreptagFix and ITCH_GreptagGluePlusLocale

    greptagdict : Dict[str, 'gbu_GrepTag'] = collections.OrderedDict()

    def __init__(self, greptag : str):
        super().__init__()
        self.GBUGT_TAG    = greptag
        self.GBUGT_LOCALE = gbu_Locale(2) if gbu_GrepTag.ENABLED else None

        assert greptag.startswith(":")
        assert greptag.endswith(":")

        if greptag in self.greptagdict:
            # ITCH_GreptagFix == y greptag helper function fail
            #     I've come to realize that the cool looking trick to build a local
            #     greptag checker function such as ... -> pyut_gmu_greptag
            #     ... is suppressing problems when the same greptag is declared
            #     elsewhere.  There's hinky and probably flawed logic to try and
            #     detect and permit a greptag to be multiply declared as long as the
            #     new declaration precisely matches the old one with respect to the
            #     specific file and line where the naked thing appears.  I didn't
            #     figure it out in a short debug try, but I'm guessing an adjustment
            #     applied by such a local checker function to propagate one more
            #     stack frame for consideration will do it.  Irritating, not evil.
            #   On attack, definitely do test cases: this has broken and swizzled
            #     enough times that it needs a real focus to muscle it down.
            #   Hmm, would passing a LOCALE from without fix this?  Adjusted?
            existing = self.greptagdict[greptag]
            matchesN = self.GBUGT_LOCALE.gbul_matchesNLevels(existing.GBUGT_LOCALE, 1) if gbu_GrepTag.ENABLED else True
            _dieUnless(matchesN, f"When considering {repr(self)} found previous {repr(existing)}")
        else:
            self.greptagdict[self.GBUGT_TAG] = self

    def __repr__(self):
        return f"<Greptag {self.GBUGT_TAG} at {str(self.GBUGT_LOCALE)}>"

    def tdugt_isSubsetOf(self, parentGrepTag : 'gbu_GrepTag'): assert False # implement if needed


# Indicates a place where a greptag is not to be used
GBU_GREPTAG_NULL = gbu_GrepTag("::")


def gbu_safeInsert(key, value, dictish):
    """If and only if the key is not present, insert it."""
    _dieIf(key in dictish, 
           lambda: f"Cannot supplant at {str(key)=} ...\n" +
                   f"  EXISTING {str(dictish[key])} \n" +
                   f"  NEW      {str(value)}")
    dictish[key] = value
    return value


def gbu_invertDict(dictionary):
    inverted_dict = collections.defaultdict(list)
    for key, value in dictionary.items():
        inverted_dict[value].append(key)
    return dict(inverted_dict)


def gbu_onlyIfCallerOneOf(*functions):
    """Throw an exception unless one of the callers of the
    current function is one of the provided functions.

    I had to generalize it to any caller since the use of
    decorators for caching means some function references
    are a step or two away.
    
    Update: For a good time see builtin -> any
    """
    # ITCH_DisableCallerAuth: THIS FUNCTION IS DISABLED FOR NOW.  
    #    There are probably better ways to do this that don't take a really long time.
    return

    all_names = set()
    for function in functions:
        next_name = function.__name__
        all_names.add(next_name)

    all_frames = []
    for record in inspect.stack():
        frame = record[0]
        info = inspect.getframeinfo(frame)
        all_frames.append(info.function)
        if info.function in all_names: return
    _dieNow(f"Cannot call this method except from {all_names}; tried from {all_frames}")

# ITCH_LaterFastFix: Promote to `gbu_need` using below
def gbu_needType(obj, neededType, doc_IGNORED=None):
    """Assert that the named object is exactly of the type specified."""
    _dieUnless(isinstance(obj, neededType),
               f"WRONG TYPE.  Needed:{neededType.__name__} but saw {obj.__class__.__name__}")
    return obj

# ITCH_LaterFastFix: consider gbu_needed for length rhyme with gbu_option
def gbu_need(obj, neededType, doc_IGNORED=None):
    return gbu_needType(obj, neededType, doc_IGNORED)

# ITCH_LaterFastFix: Promote to `gbu_optional` or `gbu_option` 
def gbu_needOptionalType(obj, neededTypeOrNone, doc_IGNORED=None):
    """Assert that the named object is of the type specified, or None allowed too."""
    if obj is None or isinstance(obj, neededTypeOrNone): return obj
    _dieNow(f"WRONG TYPE.  NeededOptional:{neededTypeOrNone.__name__} but saw {obj.__class__.__name__}")

# ITCH_LaterFastFix: Promote to `gbu_option` ?
def gbu_optional(obj, neededType, doc_IGNORED=None):
    return gbu_needOptionalType(obj, neededType, doc_IGNORED)

# ITCH_LaterFastFix: Nuke all 'assert' naming in favor of truer 'raise'... hmm old thought.  Reconsider...

def gbu_assertIsString(string):
    _dieUnless(isinstance(string, str), f"Nope not a string {string}")
    return string

def gbu_assertUnlessLowerCase(string):
    _dieIf(gbu_assertIsString(string) != string.lower(), f"Require lower case but got --> {string}")
    return string

def gbu_assertUnlessUpperCase(string):
    _dieIf(gbu_assertIsString(string) != string.upper(), f"Require upper case but got --> {string}")
    return string

def gbu_assertUnlessStartsUpperCase(longString):
    gbu_assertIsString(longString)
    string = longString[0]
    _dieIf(string.upper() != string, f"Require upper case leadin but got --> {longString}")
    return longString

def gbu_assertUnlessStartsLowerCase(longString):
    gbu_assertIsString(longString)
    string = longString[0]
    _dieIf(string.lower() != string, f"Require lower case leadin but got --> {string}")
    return longString

def gbu_assertUnlessToken(longString, cppOkay=False, optional=False):
    if optional and longString is None: return
    gbu_assertIsString(longString)
    gbu_Token.gbut_raiseUnlessValid(longString, cppOkay)
    return longString

def gbu_createCString(pythonString):
    gbu_assertIsString(pythonString)
    return "\"" + pythonString + "\"";

def gbu_diminishFirstLetter(pythonString):
    gbu_assertIsString(pythonString)
    return pythonString[0].lower() + pythonString[1:]

def gbu_raiseFirstLetter(pythonString):
    gbu_assertIsString(pythonString)
    return pythonString[0].upper() + pythonString[1:]

def gbu_assertIfComma(value):
    _dieIf(',' in value, f"This value cannot contain commas -> {value}")
    return value

def gbu_assertUnlessNone(value):
    _dieIf(value is not None, f"This value must be none but has value {value}")
    return

def gbu_checkIn(value, values):
    _dieIf(value not in values, f"Value {value} not in {values}")
    return value

def gbu_assureCallable( candidateCallable):
    _dieUnless(callable(candidateCallable), f"not callable- {candidateCallable=}")
    return              candidateCallable

def gbu_assureLambda(candidateLambda):
    _dieIf(not  callable(candidateLambda),         "Lambda must be but is not callable")
    _dieIf(candidateLambda.__name__ != '<lambda>', "Lambda is not a lambda by name. too much?")
    return candidateLambda


def gbu_bitsForValue(value):
    rv = 0
    while (1 << rv) < value: rv += 1
    return rv


def gbu_bitsForIterable(iterable, adjustValue=0):
    totalValues = adjustValue + len(iterable)
    return gbu_bitsForValue(totalValues)

def gbu_camelize(*words):  return ' '.join(word.capitalize() for word in words)

def gbu_ujoin(*words):  return GBU_UNDERSCORE.join(word for word in words)

def gbu_extractWordCaps(string):
    """For snake or camel case, extract just the first characters of each word."""
    rv = ""
    forceNext = True
    for char in string:
        if forceNext:
            rv += char
            forceNext = False
        elif char == GBU_UNDERSCORE:
            forceNext = True
        elif char == char.upper():
            rv += char
    return rv


# ITCH_PostTruthCv0: wherever this used, go and pull it down a level, until there is
#                    a good reason to split it back out.  It will be trivial to add
#                    override parameters where actually needed, which seems nowhere.
def gbu_name(string):
    """Return initializer tuple of form...
         ('Yyyy', 'yyyy')
    ...used for type/var naming.

    ITCH_ForceUpperLowerNameConv == z Simplify calling conventions, delete this
        I don't love how many places use this in combination with asterisk expansion
        but it is holding for now.
    """
    return gbu_raiseFirstLetter(string), gbu_diminishFirstLetter(string)


class gbu_Element:
    """Somewhat experimental class to get super elegant codegeneration loops."""
    def __init__(self, item, elementNumber, isFirst, isLast, isOnly):
        self.GBUE_ITEM     = item
        self.GBUE_INDEX    = elementNumber
        self.GBUE_IS_FIRST = isFirst
        self.GBUE_IS_LAST  = isLast
        self.GBUE_IS_ONLY  = isOnly

    @staticmethod
    def gbue_create(items):
        if   len(items) == 0: return tuple()
        elif len(items) == 1: return tuple([gbu_Element(items[0], 0, True, True, True)])
        itemCount = len(items)
        rv = []
        for i, item in enumerate(items):
            rv.append(gbu_Element(item, i, i == 0, i + 1 == itemCount, False))
        return tuple(rv)

def gbu_loop(items):
    return tuple([(item.GBUE_ITEM,
                   item.GBUE_INDEX,
                   item.GBUE_IS_FIRST,
                   item.GBUE_IS_LAST,
                   item.GBUE_IS_ONLY) for item in gbu_Element.gbue_create(items)])

# ITCH_ConsolidateZTagging == z ZTagging is done in too many places for private functions
def gbu_isInternalBaseName(string):
    """A longstanding convention in this codebase is that names of nonpublic
    functions and methods start with Z and are followed by an uppercase letter.

    SOMEDAY consider placing this in gchu since it is policy oriented.  Beware
    this is in conflict with the gcu_Module which has policy choices related
    to this.
    """
    
    if len(string) <= 1: return False
    first = string[0]
    second = string[1]
    if first.lower() != 'z': return False
    if second.upper() != second: return False
    return True

# ITCH_LaterFastFix: cut if not actually used anymore
def gbu_extractModuleName(headerName):
    """Given a string comprised of a module prefix and a module name separated
    by a single underscore, split and returnthe components.

    SOMEDAY consider placing this in gchu since it is policy oriented.  Beware
    this is in conflict with the gcu_Module which has policy choices related
    to this.
    """

    hsplit   = headerName.split(GBU_UNDERSCORE)
    _dieIf(len(hsplit) != 2, "Bad module name")
    prefix   = hsplit[0]
    modname  = hsplit[1].split('.')[0]
    return (prefix, modname)

def gbu_makeInlineComment(note, greptag=None):
    if note is None: note = ""
    containsInline   = "//" in note
    containsTerminal = "*/" in note
    _dieIf(containsInline or containsTerminal, f"Cannot encapsulate comment {note}")
    if greptag:
        if len(note) > 0: note = note + "   "
        note = note + greptag.GBUGT_TAG
    return "/* " + note + " */"

def gbu_varNameIterator(prefix):
    """Iterator that generates a list of temporary names"""
    step = 1
    while True:
        yield f"{prefix}{step}"
        step += 1

class gbu_OneNonnoneChecker:
    """Utility for seamlesssly checking that only one of a group of
    things are not None."""

    def __init__(self):
        self._gbuonc_count = 0

    def onc_next(self, value):
        if value is not None: self._gbuonc_count += 1
        _dieIf(self._gbuonc_count > 1, "More than one")
        return value

    def onc_batch(self, *args):
        for value in args: self.onc_next(value)


class gbu_OneTrueChecker:
    """Utility for seamlessly checking that only one of a group of
    things are not False.

    SOMEDAY look into replacing or melding with python keyword `any` and/ or
    merge with gbu_findNonNone function.
    """

    def __init__(self):
        self._gbuotc_count = 0

    def otc_next(self, value):
        if value is not False: self._gbuotc_count += 1
        _dieIf(self._gbuotc_count > 1, "More than one")
        return value

    def otc_batch(self, *args):
        for value in args: self.otc_next(value)


def gbu_findNonNone(*args):
    """Return the single non-None argument, None otherwise"""
    found = None
    for arg in args:
        if arg is not None:
            if found is not None:
                return None
            found = arg
    return found


def gbu_once(method):
    """Decorate a class method such that it is only called once for any objert
    and argument set.

    ITCH_GbuOnceOverused == z Decorator Not So Cool After All
       Now that I've made heavy use of some of the 'cool' makeXYZ operations,
       I realize that all that cleverness to never return something different
       is misplaced.  The tamer does some of this better now, and besides,
       most times a caller can call the func once and keep what it gets to reuse
       without getting ugly.  So, maybe sunset this over time?

    Use this as a decorator to indicate that the decorated method should
    only ever be evaluated once for the given object with the given arguments.
    Once evaluated a single time, the decorator assures that the return value
    is returned from a private cache rather than re-evaluating the method.

    Use this pattern when a complex resource is generated by a specific method
    which would create a conflict if attempting to create a doppelganger.

    Help from https://stackoverflow.com/questions/7473096/python-decorators-how-to-use-parent-class-decorators-in-a-child-class

    POTENTIAL FEATURE: Add a 'query' for the global dictionary that is used to
    decide whether a particular wrapping is allowed to come into existance or
    to be returned.  I'm thinking of the case where I want to be assured that
    the returned object has precisely matching optional fields or not.  I think
    I can synthesize this with the non-mutating query which determines whether
    a prior cached object passes additional integrity checks before actually
    returning said prior cached object.  My specific case is the unusual provision
    of documentation text that adds human understanding constraints to a returned
    object- such constraints cannot be observed unless all uses of the object
    acquiesce to them, and no uses are permitted to come into existence where
    such a promise cannot be kept.  Taking on this feature will **require** some
    test cases be constructed and observed, and that's the main reason I'm setting
    aside now.  LATE ADD: the case of giu_Element use may blow all this up!
    Approach with caution.
    """

    def _gbuc_wrapper(self, *args):
        key = (method.__name__, self, *args)
        if not key in _globalDict:
            val = method(self, *args)
            _globalDict[key] = val
        return _globalDict[key]
    return _gbuc_wrapper

# Dictionary used to cache function return values
_globalDict = collections.OrderedDict()


#
# Somewhat inspired by https://stackoverflow.com/questions/4005318/how-to-implement-a-good-hash-function-in-python
class gbu_Hasher:
    f"""Really only for use when you don't need a good hash.  Related strongly 
    to shoehorning {gbu_once} usages where precise equality isn't necessary."""

    def __init__(self):
        self.__gbuh_raw = []

    def gbuh_add(self, item):
        self.__gbuh_raw.append(item)
        return item

    def gbuh_hash(self):
        tpl = tuple(self.__gbuh_raw)
        self.__gbuh_raw = None
        return hash(tpl)


class gbu_Obsolete:
    """Formerly, CharacterCount flyweight object.  Not pythonic.  Retaining code
    for potential interesting ideas if issue returns.

    Stores a constant equal to a number of characters. 

    You must acquire instances through the factory method.  This factory
    method assures that there is only one object created per raw number which
    is returned.  This is 'Flyweight' pattern.  This directly supports use
    as a key in a hash table.

    Caveat: I know this is not pythonic but once a parameter type gets used
    in enough places, this helps me.  This is especially true due to the 
    concept confusion where I use the concept of a 'column' in two very
    related, very different ways.

    MAYBE SOMEDAY figure out how to create a gbu_Flyweight base class here
    """

    def __init__(self, value): 
        _dieUnless(isinstance(value, int), f"Bad type creation here, parameter is {value}")
        self.FTUCC_VALUE = value

    def __setattr__(self, name, value):
        isInitCriteria = sys._getframe(2).f_code.co_name == 'tlucc_acquireUnique'
        _dieUnless(isInitCriteria, "Cannot do this unless we are in the factory method")
        self.__dict__[name] = value

    def __str__(self): return "{CharacterCount:%s}" % self.FTUCC_VALUE

    tlucc_instanceDict : Dict[int, 'gbu_Obsolete'] = {}

    @classmethod
    def tlucc_acquireUnique(cls, value : int):
        if value not in cls.tlucc_instanceDict:
               cls.tlucc_instanceDict[value] = gbu_Obsolete(value)
        return cls.tlucc_instanceDict[value]


def gbu_makeKwargs(**kwargs):
    """Goofy little utility that lets me form kwargs without string names.  The voices..."""
    return kwargs


class gbu_MultiplexBase(gbu_Immutable):
    """
    I've decided to do A Bad Thing.  I find I need to handle multiple similar language
    variants and I'm going to do something sort of evil here.  Mostly this area of code
    is rarely touched so it should be limited.

    The bad thing is that I'll use kwargs to indicate what languages a particular 
    definition can be used for.
    """

    def __init__(self, subpartList, **kwargs):
        super().__init__()

        self.__raw_kwargs = kwargs
        self.__verifier = gbu_IceDictionary(str, str)
        for key, value in kwargs.items():
            for root in subpartList:
                if root in key:
                    # If one of these throws, the language variants are overspecified.  Try again!
                    self.__verifier[root] = value

    def gbum_getBySubpart(self, subpart):
        _dieIf(subpart not in self.__verifier,
               f"No valid option seeking {subpart} given kwargs {self.__raw_kwargs}")
        return self.__verifier[subpart]


class gbu_MultiplexCish(gbu_MultiplexBase):
    """Specialization of a kwargs demultiplex appropriate for encoding C
    versus C++ versus both as appropriate.
    """

    ARGCODE_CXX  = 'cxx'  # FOR C NAMES
    ARGCODE_CPP  = 'cpp'  # FOR C++ NAMES

    def __init__(self, **kwargs):
        super().__init__([self.ARGCODE_CXX, self.ARGCODE_CPP], **kwargs)

    def gbumc_getCorrectFlavor(self, is_cpp):
        return self.gbum_getBySubpart(self.ARGCODE_CPP) if is_cpp else \
               self.gbum_getBySubpart(self.ARGCODE_CXX)


def gbu_zTag(string): return f"z{string}"

# ITCH_gbuClearDirImprove == z single point of sophistication for emptying dir well.
#    This is sophisticated because unit testing is hard, so I trust history.  But then
#    it has gone many places.  What to do to make this critical function solid?  Also
#    note briefly the oddities that this is called inside of utilities as well as outside
#    in the bash build systems- very hetero use...
def gbu_safeClearDirectory(basepath):
    """See tdui_commitToFilesystem for place of origin, and related
    to the tste_ToolsSafeTreeEmpty.py app."""
    
    def __log(line): gbu_trace(line, frameNudge=1)
    __log("Start...")

    __log(f"Delete all files at {basepath=}")
    for root, dirs, files in os.walk(basepath, topdown=False):
        for name in files:
            existFileName = os.path.join(root, name)
            __log(f"deleting file {existFileName=}")
            os.remove(existFileName)

    __log(f"Unlink all directories at {basepath=}")
    for dirpath, dirnames, filenames in os.walk(basepath, topdown=False):
        for dirname in dirnames:
            directory = os.path.join(dirpath, dirname)
            try:
                # Attempt to remove the subdirectory
                os.rmdir(directory)
                __log(f"Deleted directory: {directory}")
            except OSError as e:
                __log(f"Error deleting directory: {directory} - {e}")
                _dieNow("I think this is bad, but not sure")


# ITCH_MarkdownImprovements == z Make markdown nicer
# ITCH_MarkdownImprovements: consider gmu_GenerateMarkdownUtility as holder for local policy            
# ITCH_MarkdownImprovements: Make builder class
# ITCH_MarkdownImprovements: after builder, add foldable sections as per ->
#               https://gist.github.com/pierrejoubert73/902cc94d79424356a8d20be2b382e1ab
# ITCH_MarkdownImprovements: consider tuple return, and rolling all way up to string builders
def gbu_mdHeading(headingDepth, string):
    prefix = '#' * headingDepth
    return f"{prefix} {string}"


def gbu_mdBullet(bulletDepth, string):
    prefix = '  ' * bulletDepth
    return f"{prefix}* {string}"

def gbu_mdSymbol(string):
    return f"`{string}`"


