# Robusta (CAD Plan Checker Utility)
## Brief
Given station, bearing, line data information on plan, if a program can reconstruct the line work then it should be proof of constructability.

## Todo!()
- Reconstruct horizontal linework given a table (what)
- linear algebra library thru nalgebra, faer, or cgmath. 
    nalgebra - general purpose 
    faer - large matrix operation
    **cgmath - general purpose, easier to use angle trait. **
        https://github.com/rustgd/cgmath
        what is swizzling lol 
        --features mint for matrix interpo between libraries.
- Move dependencies from term to other parts of the codebase. 
- Use bevy in -gui to build the frontend. 
- egui and `cargo add bevy_pancam --features bevy_egui`
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
