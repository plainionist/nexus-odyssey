
Using the power of graphs and 3D rendering to structure ideas

# Json format

```json
{
  "nodes": [
    { "id": "A", "group": 1 },
    { "id": "B", "group": 1 },
    { "id": "C", "group": 1 }
  ],
  "links": [
    { "source": "A", "target": "B" },
    { "source": "B", "target": "C" },
    { "source": "C", "target": "A" }
  ]
}
```

# Further thoughts

- folder structure
  - folder structure usually has a purpose, it gives certain meaningful structure
  - it should also contribute to connecting the nodes
  - but not all folders have that purpose - maybe skip top level?

- a single file might not be the smallest unit
  - e.g. a blog post is a single file but may contain sub-topics which we want to represent separately
  - e.g. the article could list 5 design patterns
  - how do we identify those sections?
    - not every headline has that purpose (e.g. introduction & conclusion are not separate topics)
    - headlines are probably a good criteria but we need "markers" for such sections
  - also create automatic links from sections to main document
  - so a doc listing 5 patterns results in 6 nodes: one for the doc + 5 for each pattern
    (and might have more sections/headlines)

- interactive filter of nodes in search

- focus mode: "show siblings only" (see mind-elixir)
  - https://vasturiano.github.io/3d-force-graph/example/click-to-focus/
  - https://github.com/vasturiano/3d-force-graph/blob/master/example/click-to-focus/index.html

# References

- https://github.com/vasturiano/3d-force-graph
