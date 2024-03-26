# Robusta (CAD Plan Checker Utility)
## Brief
Given station, bearing, line data information on plan, if a program can reconstruct the line work then it should be proof of constructability.

## Ui Overhaul
- The Ui will be a hybrid of Helix Editor and a standard gui-based editing surface. Experienced users will benefit from the mouse-less keybindings and beginners will benefit from the CADPanel. 

## Rewrite 2 Update
- Take advantage of Bevy ECS change detection (https://docs.rs/bevy_ecs/latest/bevy_ecs/#change-detection). 
- Constraints: add coincident & extension constriant type.
- Entity behavior
    click once: select the entity/place the point. bring up the constraints imposed on this entity. highlight the entity depending on if it's a fixed for floating entity. (entities that can have fixed and floating points use gradients.) 
    click again on a definition point: If it's a fixed point, pick-up the point and move it to the mouse for dropping. If it's a float point, it bring up the constraints to be removed or converted to fixed point. 
        Intersections:
        fixed-fixed > pick-up the point belonging to the line/arc selected. No constraints can be added.
        fixed-float > moving the fixed will also move the float, maintaining constraints. 
        float-float > shouldn't be possible as it's underdefined. (untested)
- Take into consideration the following:
    - Component Storage type
    - Component Bundles for organization
- Phantom system is split into the pointer and the phantom entities. This is required for the function of multi-point definition entities. 

## Proposed Re-write 2
- Term, dxf, core, bevy_egui are all fine. gui is the one that requires a re-write.
- Use more of the ECS system by adding RobustaEntity into the entity itself via component. This way, Hashmap isn't required to correlate the two. 
- Take advantage of Bevy ECS change detection (https://docs.rs/bevy_ecs/latest/bevy_ecs/#change-detection). 
- Take into consideration the following:
    - Component Storage type
    - Component Bundles for organization
- Break down systems into smaller chunks, then organize into plugins. Model after either bevy_egui or bevy_mod_picking.
    - This is to increase the modularity, and decrease the accumulation of code debt. This will also contribute to reducing update cycle bloat. 
- Since egui is immediate mode, the entire cadpanel will be re-drawn every frame.
- Constraints: add coincident & extension constriant type.
- Entity behavior
    click once: select the entity/place the point. bring up the constraints imposed on this entity. highlight the entity depending on if it's a fixed for floating entity. (entities that can have fixed and floating points use gradients.) 
    click again on a definition point: If it's a fixed point, pick-up the point and move it to the mouse for dropping. If it's a float point, it bring up the constraints to be removed or converted to fixed point. 
        Intersections:
        fixed-fixed > pick-up the point belonging to the line/arc selected. No constraints can be added.
        fixed-float > moving the fixed will also move the float, maintaining constraints. 
        float-float > shouldn't be possible as it's underdefined. (untested)
    

## Re-write 1
- The primary window will host all the viewports and cameras.
- The 2nd window will host the egui docking controlboard. 


## Todo!()
- clone https://github.com/johanhelsing/bevy_pancam and change world_units_per_device_pixel so horizontal and vertical scrolling is the same. Can also fix some other Issues if it's easy. 
- add a 'fit to view' function
- linear algebra library thru nalgebra, faer, or cgmath. 
    nalgebra - general purpose 
    faer - large matrix operation
    **cgmath - general purpose, easier to use angle trait. **
        https://github.com/rustgd/cgmath
        what is swizzling lol 
        --features mint for matrix interpo between libraries.
- multiple viewports
- impliment and undo and redo system. The easiest way is to save the global resource (uistate) into a buffer with ~20 or so layers.
- instead of using bevy gizmos, upgrade to robusta entities when more control is required.

## Brainstorming
- Meta-entities
Everything is made of simple points. Since everything is simple geometry, the functions will be limited. To increase capability, "lesser" geometry can be "tagged" with meta geometry like complexes (this is a meta-geometry comprised of a series of lines and arcs), alignments, maybe even lines and arcs.
    Meta-entities are divided into different types:
        lines & arcs - Inbetween points ..
        polylines, alignments - 

- Tags
Playing on the idea if meta-entities, tags like polyline, alignments, line, arc are 
    Use the Bevy ECS system alongside tags.

- Alignments
Technically this can be built with JUST lines or arcs, but for the sake of design it's complexes only. 

- Visualization
Points that assume multiple tags are visualized with dynamic features like color phasing? Depending on the color scheme/config file, a point that is a parent to a line will phase blue.. Color phasing is dumb. 

- Constraint System
This is important, but for another day. 

- Fence System
In addition to the constraint system, have it so arcs and lines must be between two points. This is called the fence system as of now.

---

## Bugs
- On startup, using keys: [win + arrow keys] messes with the bevy focus system and app functionality is impacted. This is an upstream issue.
- With [select nothing deselect all] on, clicking on the cadpanel deselects all even though the settings are changed. 

## QOL Features
- Impliment a fuzzy-finder to the RTerm and its dictionary.
- Icons to the constraints on the ribbon.
- Similar to how helix/vim gives context clues on what mode you are in, give clues to what you are doing. Might be a good design philosophy where you are able to determine what is going on based on a screenshot alone.

## Alignment of Text
Horizontal:
    Left, snap point is SW corner
    Center, snap point is S middle
    Right, snap point is SE corner
    Aligned, no movement, it's two points to define the text? basically, convert to "left"
    Middle, snap point is in the exact middle of horizontal and vertical
    Fit, same as aligned, but slightly different.

Vertical:
    Baseline, this takes into account the font passsing through the x-axis. It's most common.
    Bottom,
    Middle,
    Top,

## Tag Behavior
Undefined behavior like an entity without any tags (including Default), should prevent the CAD from compiling. This is to prevent human errors from plaguing the design file. 

A compiling design file must possess the following:
1. No untagged entities.