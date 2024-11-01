//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSArray<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSCopying for NSArray<ObjectType> {}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSArray<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSMutableCopying for NSArray<ObjectType> {}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSArray<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding for NSArray<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method(count)]
        pub fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectAtIndex:)]
        pub unsafe fn objectAtIndex(&self, index: NSUInteger) -> Retained<ObjectType>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Allocated<Self>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl<ObjectType: Message> DefaultRetained for NSArray<ObjectType> {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBinarySearchingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSBinarySearchingOptions: NSUInteger {
        const NSBinarySearchingFirstEqual = 1<<8;
        const NSBinarySearchingLastEqual = 1<<9;
        const NSBinarySearchingInsertionIndex = 1<<10;
    }
}

unsafe impl Encode for NSBinarySearchingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBinarySearchingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSExtendedArray
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(@__retain_semantics Other arrayByAddingObject:)]
        pub unsafe fn arrayByAddingObject(
            &self,
            an_object: &ObjectType,
        ) -> Retained<NSArray<ObjectType>>;

        #[method_id(@__retain_semantics Other arrayByAddingObjectsFromArray:)]
        pub unsafe fn arrayByAddingObjectsFromArray(
            &self,
            other_array: &NSArray<ObjectType>,
        ) -> Retained<NSArray<ObjectType>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other componentsJoinedByString:)]
        pub unsafe fn componentsJoinedByString(&self, separator: &NSString) -> Retained<NSString>;

        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, an_object: &ObjectType) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(
            &self,
            locale: Option<&AnyObject>,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:indent:)]
        pub unsafe fn descriptionWithLocale_indent(
            &self,
            locale: Option<&AnyObject>,
            level: NSUInteger,
        ) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other firstObjectCommonWithArray:)]
        pub unsafe fn firstObjectCommonWithArray(
            &self,
            other_array: &NSArray<ObjectType>,
        ) -> Option<Retained<ObjectType>>;

        #[cfg(feature = "NSRange")]
        #[method(getObjects:range:)]
        pub unsafe fn getObjects_range(
            &self,
            objects: NonNull<NonNull<ObjectType>>,
            range: NSRange,
        );

        #[method(indexOfObject:)]
        pub unsafe fn indexOfObject(&self, an_object: &ObjectType) -> NSUInteger;

        #[cfg(feature = "NSRange")]
        #[method(indexOfObject:inRange:)]
        pub unsafe fn indexOfObject_inRange(
            &self,
            an_object: &ObjectType,
            range: NSRange,
        ) -> NSUInteger;

        #[method(indexOfObjectIdenticalTo:)]
        pub unsafe fn indexOfObjectIdenticalTo(&self, an_object: &ObjectType) -> NSUInteger;

        #[cfg(feature = "NSRange")]
        #[method(indexOfObjectIdenticalTo:inRange:)]
        pub unsafe fn indexOfObjectIdenticalTo_inRange(
            &self,
            an_object: &ObjectType,
            range: NSRange,
        ) -> NSUInteger;

        #[method(isEqualToArray:)]
        pub unsafe fn isEqualToArray(&self, other_array: &NSArray<ObjectType>) -> bool;

        #[method_id(@__retain_semantics Other firstObject)]
        pub unsafe fn firstObject(&self) -> Option<Retained<ObjectType>>;

        #[method_id(@__retain_semantics Other lastObject)]
        pub unsafe fn lastObject(&self) -> Option<Retained<ObjectType>>;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Retained<NSEnumerator<ObjectType>>;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other reverseObjectEnumerator)]
        pub unsafe fn reverseObjectEnumerator(&self) -> Retained<NSEnumerator<ObjectType>>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other sortedArrayHint)]
        pub unsafe fn sortedArrayHint(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other sortedArrayUsingFunction:context:)]
        pub unsafe fn sortedArrayUsingFunction_context(
            &self,
            comparator: unsafe extern "C" fn(
                NonNull<ObjectType>,
                NonNull<ObjectType>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
        ) -> Retained<NSArray<ObjectType>>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other sortedArrayUsingFunction:context:hint:)]
        pub unsafe fn sortedArrayUsingFunction_context_hint(
            &self,
            comparator: unsafe extern "C" fn(
                NonNull<ObjectType>,
                NonNull<ObjectType>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
            hint: Option<&NSData>,
        ) -> Retained<NSArray<ObjectType>>;

        #[method_id(@__retain_semantics Other sortedArrayUsingSelector:)]
        pub unsafe fn sortedArrayUsingSelector(
            &self,
            comparator: Sel,
        ) -> Retained<NSArray<ObjectType>>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other subarrayWithRange:)]
        pub unsafe fn subarrayWithRange(&self, range: NSRange) -> Retained<NSArray<ObjectType>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method(writeToURL:error:_)]
        pub unsafe fn writeToURL_error(&self, url: &NSURL) -> Result<(), Retained<NSError>>;

        #[method(makeObjectsPerformSelector:)]
        pub unsafe fn makeObjectsPerformSelector(&self, a_selector: Sel);

        #[method(makeObjectsPerformSelector:withObject:)]
        pub unsafe fn makeObjectsPerformSelector_withObject(
            &self,
            a_selector: Sel,
            argument: Option<&AnyObject>,
        );

        #[cfg(feature = "NSIndexSet")]
        #[method_id(@__retain_semantics Other objectsAtIndexes:)]
        pub unsafe fn objectsAtIndexes(
            &self,
            indexes: &NSIndexSet,
        ) -> Retained<NSArray<ObjectType>>;

        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(&self, idx: NSUInteger) -> Retained<ObjectType>;

        #[cfg(feature = "block2")]
        #[method(enumerateObjectsUsingBlock:)]
        pub unsafe fn enumerateObjectsUsingBlock(
            &self,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(enumerateObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NSIndexSet", feature = "NSObjCRuntime", feature = "block2"))]
        #[method(enumerateObjectsAtIndexes:options:usingBlock:)]
        pub unsafe fn enumerateObjectsAtIndexes_options_usingBlock(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) + '_>,
        );

        #[cfg(feature = "block2")]
        #[method(indexOfObjectPassingTest:)]
        pub unsafe fn indexOfObjectPassingTest(
            &self,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(indexOfObjectWithOptions:passingTest:)]
        pub unsafe fn indexOfObjectWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSIndexSet", feature = "NSObjCRuntime", feature = "block2"))]
        #[method(indexOfObjectAtIndexes:options:passingTest:)]
        pub unsafe fn indexOfObjectAtIndexes_options_passingTest(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSIndexSet", feature = "block2"))]
        #[method_id(@__retain_semantics Other indexesOfObjectsPassingTest:)]
        pub unsafe fn indexesOfObjectsPassingTest(
            &self,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> Retained<NSIndexSet>;

        #[cfg(all(feature = "NSIndexSet", feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other indexesOfObjectsWithOptions:passingTest:)]
        pub unsafe fn indexesOfObjectsWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> Retained<NSIndexSet>;

        #[cfg(all(feature = "NSIndexSet", feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other indexesOfObjectsAtIndexes:options:passingTest:)]
        pub unsafe fn indexesOfObjectsAtIndexes_options_passingTest(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> Retained<NSIndexSet>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other sortedArrayUsingComparator:)]
        pub unsafe fn sortedArrayUsingComparator(
            &self,
            cmptr: NSComparator,
        ) -> Retained<NSArray<ObjectType>>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other sortedArrayWithOptions:usingComparator:)]
        pub unsafe fn sortedArrayWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        ) -> Retained<NSArray<ObjectType>>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRange", feature = "block2"))]
        #[method(indexOfObject:inSortedRange:options:usingComparator:)]
        pub unsafe fn indexOfObject_inSortedRange_options_usingComparator(
            &self,
            obj: &ObjectType,
            r: NSRange,
            opts: NSBinarySearchingOptions,
            cmp: NSComparator,
        ) -> NSUInteger;
    }
);

extern_methods!(
    /// NSArrayCreation
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(@__retain_semantics Other array)]
        pub unsafe fn array() -> Retained<Self>;

        #[method_id(@__retain_semantics Other arrayWithObject:)]
        pub unsafe fn arrayWithObject(an_object: &ObjectType) -> Retained<Self>;

        #[method_id(@__retain_semantics Other arrayWithObjects:count:)]
        pub unsafe fn arrayWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other arrayWithArray:)]
        pub unsafe fn arrayWithArray(array: &NSArray<ObjectType>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Allocated<Self>,
            array: &NSArray<ObjectType>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            this: Allocated<Self>,
            array: &NSArray<ObjectType>,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Result<Retained<NSArray<ObjectType>>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other arrayWithContentsOfURL:error:_)]
        pub unsafe fn arrayWithContentsOfURL_error(
            url: &NSURL,
        ) -> Result<Retained<NSArray<ObjectType>>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSArray`
    ///
    /// NSArrayCreation
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method_id(@__retain_semantics Other array)]
        pub unsafe fn array() -> Retained<Self>;

        #[method_id(@__retain_semantics Other arrayWithObject:)]
        pub unsafe fn arrayWithObject(an_object: &ObjectType) -> Retained<Self>;

        #[method_id(@__retain_semantics Other arrayWithObjects:count:)]
        pub unsafe fn arrayWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other arrayWithArray:)]
        pub unsafe fn arrayWithArray(array: &NSArray<ObjectType>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Allocated<Self>,
            array: &NSArray<ObjectType>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            this: Allocated<Self>,
            array: &NSArray<ObjectType>,
            flag: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSArrayDiffing
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[cfg(all(feature = "NSOrderedCollectionDifference", feature = "block2"))]
        #[method_id(@__retain_semantics Other differenceFromArray:withOptions:usingEquivalenceTest:)]
        pub unsafe fn differenceFromArray_withOptions_usingEquivalenceTest(
            &self,
            other: &NSArray<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NonNull<ObjectType>) -> Bool + '_>,
        ) -> Retained<NSOrderedCollectionDifference<ObjectType>>;

        #[cfg(feature = "NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromArray:withOptions:)]
        pub unsafe fn differenceFromArray_withOptions(
            &self,
            other: &NSArray<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
        ) -> Retained<NSOrderedCollectionDifference<ObjectType>>;

        #[cfg(feature = "NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromArray:)]
        pub unsafe fn differenceFromArray(
            &self,
            other: &NSArray<ObjectType>,
        ) -> Retained<NSOrderedCollectionDifference<ObjectType>>;

        #[cfg(feature = "NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other arrayByApplyingDifference:)]
        pub unsafe fn arrayByApplyingDifference(
            &self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        ) -> Option<Retained<NSArray<ObjectType>>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[deprecated = "Use -getObjects:range: instead"]
        #[method(getObjects:)]
        pub unsafe fn getObjects(&self, objects: NonNull<NonNull<ObjectType>>);

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other arrayWithContentsOfFile:)]
        pub unsafe fn arrayWithContentsOfFile(
            path: &NSString,
        ) -> Option<Retained<NSArray<ObjectType>>>;

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Other arrayWithContentsOfURL:)]
        pub unsafe fn arrayWithContentsOfURL(url: &NSURL) -> Option<Retained<NSArray<ObjectType>>>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<NSArray<ObjectType>>>;

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Option<Retained<NSArray<ObjectType>>>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            use_auxiliary_file: bool,
        ) -> bool;

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSMutableArray<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSCopying for NSMutableArray<ObjectType> {}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSMutableArray<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSMutableCopying for NSMutableArray<ObjectType> {}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSMutableArray<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding for NSMutableArray<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method(addObject:)]
        pub unsafe fn addObject(&mut self, an_object: &ObjectType);

        #[method(insertObject:atIndex:)]
        pub unsafe fn insertObject_atIndex(&mut self, an_object: &ObjectType, index: NSUInteger);

        #[method(removeLastObject)]
        pub unsafe fn removeLastObject(&mut self);

        #[method(removeObjectAtIndex:)]
        pub unsafe fn removeObjectAtIndex(&mut self, index: NSUInteger);

        #[method(replaceObjectAtIndex:withObject:)]
        pub unsafe fn replaceObjectAtIndex_withObject(
            &mut self,
            index: NSUInteger,
            an_object: &ObjectType,
        );

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Allocated<Self>,
            num_items: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSArray`
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Allocated<Self>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl<ObjectType: Message> DefaultRetained for NSMutableArray<ObjectType> {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedMutableArray
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method(addObjectsFromArray:)]
        pub unsafe fn addObjectsFromArray(&mut self, other_array: &NSArray<ObjectType>);

        #[method(exchangeObjectAtIndex:withObjectAtIndex:)]
        pub unsafe fn exchangeObjectAtIndex_withObjectAtIndex(
            &mut self,
            idx1: NSUInteger,
            idx2: NSUInteger,
        );

        #[method(removeAllObjects)]
        pub fn removeAllObjects(&mut self);

        #[cfg(feature = "NSRange")]
        #[method(removeObject:inRange:)]
        pub unsafe fn removeObject_inRange(&mut self, an_object: &ObjectType, range: NSRange);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&mut self, an_object: &ObjectType);

        #[cfg(feature = "NSRange")]
        #[method(removeObjectIdenticalTo:inRange:)]
        pub unsafe fn removeObjectIdenticalTo_inRange(
            &mut self,
            an_object: &ObjectType,
            range: NSRange,
        );

        #[method(removeObjectIdenticalTo:)]
        pub unsafe fn removeObjectIdenticalTo(&mut self, an_object: &ObjectType);

        #[deprecated = "Not supported"]
        #[method(removeObjectsFromIndices:numIndices:)]
        pub unsafe fn removeObjectsFromIndices_numIndices(
            &mut self,
            indices: NonNull<NSUInteger>,
            cnt: NSUInteger,
        );

        #[method(removeObjectsInArray:)]
        pub unsafe fn removeObjectsInArray(&mut self, other_array: &NSArray<ObjectType>);

        #[cfg(feature = "NSRange")]
        #[method(removeObjectsInRange:)]
        pub unsafe fn removeObjectsInRange(&mut self, range: NSRange);

        #[cfg(feature = "NSRange")]
        #[method(replaceObjectsInRange:withObjectsFromArray:range:)]
        pub unsafe fn replaceObjectsInRange_withObjectsFromArray_range(
            &mut self,
            range: NSRange,
            other_array: &NSArray<ObjectType>,
            other_range: NSRange,
        );

        #[cfg(feature = "NSRange")]
        #[method(replaceObjectsInRange:withObjectsFromArray:)]
        pub unsafe fn replaceObjectsInRange_withObjectsFromArray(
            &mut self,
            range: NSRange,
            other_array: &NSArray<ObjectType>,
        );

        #[method(setArray:)]
        pub unsafe fn setArray(&mut self, other_array: &NSArray<ObjectType>);

        #[method(sortUsingFunction:context:)]
        pub unsafe fn sortUsingFunction_context(
            &mut self,
            compare: unsafe extern "C" fn(
                NonNull<ObjectType>,
                NonNull<ObjectType>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
        );

        #[method(sortUsingSelector:)]
        pub unsafe fn sortUsingSelector(&mut self, comparator: Sel);

        #[cfg(feature = "NSIndexSet")]
        #[method(insertObjects:atIndexes:)]
        pub unsafe fn insertObjects_atIndexes(
            &mut self,
            objects: &NSArray<ObjectType>,
            indexes: &NSIndexSet,
        );

        #[cfg(feature = "NSIndexSet")]
        #[method(removeObjectsAtIndexes:)]
        pub unsafe fn removeObjectsAtIndexes(&mut self, indexes: &NSIndexSet);

        #[cfg(feature = "NSIndexSet")]
        #[method(replaceObjectsAtIndexes:withObjects:)]
        pub unsafe fn replaceObjectsAtIndexes_withObjects(
            &mut self,
            indexes: &NSIndexSet,
            objects: &NSArray<ObjectType>,
        );

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(&mut self, obj: &ObjectType, idx: NSUInteger);

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(sortUsingComparator:)]
        pub unsafe fn sortUsingComparator(&mut self, cmptr: NSComparator);

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(sortWithOptions:usingComparator:)]
        pub unsafe fn sortWithOptions_usingComparator(
            &mut self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        );
    }
);

extern_methods!(
    /// NSMutableArrayCreation
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method_id(@__retain_semantics Other arrayWithCapacity:)]
        pub unsafe fn arrayWithCapacity(num_items: NSUInteger) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other arrayWithContentsOfFile:)]
        pub unsafe fn arrayWithContentsOfFile(
            path: &NSString,
        ) -> Option<Retained<NSMutableArray<ObjectType>>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other arrayWithContentsOfURL:)]
        pub unsafe fn arrayWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Retained<NSMutableArray<ObjectType>>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<NSMutableArray<ObjectType>>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Option<Retained<NSMutableArray<ObjectType>>>;
    }
);

extern_methods!(
    /// NSMutableArrayDiffing
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[cfg(feature = "NSOrderedCollectionDifference")]
        #[method(applyDifference:)]
        pub unsafe fn applyDifference(
            &mut self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        );
    }
);