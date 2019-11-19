pub mod data;
pub mod delta;
pub mod deltalizeable;
pub mod drill;
pub mod entrylayer;
pub mod index;
pub mod layer;
pub mod lodarea;
pub mod lodpos;
pub mod materializeable;
pub mod traversable;
pub use data::{HashLayer, HashNestLayer, VecLayer, VecNestLayer};

/*
 Terminology:
 - Layer: the layer of the LoDTree, a tree can have n layers, every layer contains their child layer, except for the last one.
          Each layer contains a level, a number from 15-0. the level of each child must be lower than the parents layer!
 - Detail: Each Layer contains information about that layer, here called Detail. This is the usable information we want to store in the LoDTree
 - LodPos: A LodPos marks a specific position inside the LoDTree, but not their layer.
           Each Detail has a LodPos. Multiple Details can exist at the same LodPos on different layers!
 - Index: This refers to the actually storage for the index for the next layer (often a u16,u32).
          The Index is used to find the child in a spare storage.
 - Key: always refers to the storage of a LAYER. Any keyword with KEY is either of type usize or LodPos.

 traits:
 - Layer: Every layer must implement this. KEY is the storage Type and either usize/LodPos. Layer is also defined here.
 - ParentLayer: Is a Layer that contains a CHILD layer and some const properties
 - IndexStore: Every layer must implement this for their Layer::KEY and INDEX is often u16/u32.
               The index is accessed by this layer to get the corresponding child.
               Every Indexstore is a ParentLayer.
 - DetailStore: Every layer must implement this for their KEY.
                This is used to store the actual DETAIL of every layer.
 - DetailStoreMut: allows mut borrow for Vec types (Hash not supported)
 !!Calculations will be implemented on these 2 Stores, rather than the actual structs to reduce duplciate coding!!
 - ToOptionUsize: to store INDEX in z16/u32 efficiently and move up to usize on calculation
 - Traversable: trait is used to get child layer and child Index for a concrete position.
 - Materializeable: trait is used to actually return a Detail for a concrete position.
 - EntryLayer: the topmost layer which can generate a Traversable for a LodPos must implement this, e.g. needed by delta

 Actual structs regarding of position in the chain. They represent the Layers and contain the Details, they implement (some of) the 2 Store traits
 Naming Scheme is <Own Detail Type>[Nest]Layer
 - VecLayer: KEY=usize, stores in Vec, leaf layer
 - HashLayer:KEY=LodPos, stores in Vec, leaf layer
 - VecNestLayer: KEY=usize, stores in Vec, has childs
 - HashNestLayer: KEY=LodPos, stores in Vec, has childs

 Result Structs:
 - HashIter/VecIter: Is used to access a layer meta information or Detail via LoDTree.trav().get().get().get().mat().
                     When LoDTree.trav() returns a HashIter.
                     It keeps information to next layer to not recalculate it
*/
