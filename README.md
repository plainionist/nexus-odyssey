
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

- interactive filter of nodes in search
- focus mode: "show siblings only" (see mind-elixir)
  - https://vasturiano.github.io/3d-force-graph/example/click-to-focus/
  - https://github.com/vasturiano/3d-force-graph/blob/master/example/click-to-focus/index.html

# References

- https://github.com/vasturiano/3d-force-graph
