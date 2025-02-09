import { ForceGraph3DInstance, Graph, GraphLink, GraphNode } from '3d-force-graph'

export function useHighlight(self: ForceGraph3DInstance) {
  const highlightNodes = new Set()
  const highlightLinks = new Set()
  let hoverNode: GraphNode | null = null

  function addNeighbors(graph: Graph) {
    const nodeMap = Object.fromEntries(graph.nodes.map((node) => [node.id, node]))

    graph.links.forEach((link) => {
      const a = nodeMap[link.source]
      const b = nodeMap[link.target]

      a.neighbors = a.neighbors || []
      b.neighbors = b.neighbors || []
      a.neighbors.push(b)
      b.neighbors.push(a)

      a.links = a.links || []
      b.links = b.links || []
      a.links.push(link)
      b.links.push(link)
    })
  }

  function updateHighlight() {
    // trigger update of highlighted objects in scene
    self.nodeColor(self.nodeColor()).linkWidth(self.linkWidth()).linkDirectionalParticles(self.linkDirectionalParticles())
  }

  function highlightNode(node?: GraphNode) {
    // no state change
    if ((!node && !highlightNodes.size) || (node && hoverNode === node)) return

    highlightNodes.clear()
    highlightLinks.clear()

    if (node && node.neighbors) {
      highlightNodes.add(node)
      node.neighbors.forEach((neighbor: any) => highlightNodes.add(neighbor))
      node.links.forEach((link: any) => highlightLinks.add(link))
    }

    hoverNode = node || null

    updateHighlight()
  }

  function highlightLink(link?: GraphLink) {
    highlightNodes.clear()
    highlightLinks.clear()

    if (link) {
      highlightLinks.add(link)
      highlightNodes.add(link.source)
      highlightNodes.add(link.target)
    }

    updateHighlight()
  }

  function getNodeColor(node: GraphNode) {
    return highlightNodes.has(node) ? (node === hoverNode ? 'rgb(255,0,0,1)' : 'rgba(255,160,0,0.8)') : 'rgba(0,255,255,0.6)'
  }

  function isHighlighted(link: GraphLink) {
    return highlightLinks.has(link)
  }

  return {
    addNeighbors,
    highlightNode,
    highlightLink,
    getNodeColor,
    isHighlighted
  }
}
