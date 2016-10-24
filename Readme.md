About
========================
This is converter from COLLADA format to model format of game engine. It tries to allow user to specify how much parameters in editor as possible.

Recommended 3D editor
========================
Blender

Technical details
========================
Meshes have just one material, that's why meshes from COLLADA file, that have >1 materials will be splitted a few meshes, their names will be like (mesh name).(material name)
The information, what contains the vertex of mesh store in vertexSemantics field. LODs (Level Of Details) contain distance, between which and distance of next LOD it's geometry should be rendered, and Vertices data, serialized in base64, (u64 as length, and data). Output format has JSON-like syntax, so it can be edited by any text editor you want.

Why not ColladaDOM?
========================
ColladaDOM is very old library, it does not develope since 2008, and it's hard to compile it's source code, you must edit it's src, and it is real horror, but COLLADA is XML format and it is easy to parse, we need just few it's features? =)
