<template>
  <main class="container">
    <div ref="canvas" id="canvas"></div>
  </main>
</template>

<script setup lang="ts">
  import { onMounted, ref } from 'vue'
  import { listen } from '@tauri-apps/api/event'
  import ForceGraph3D, { ForceGraph3DInstance, Graph, GraphLink, GraphNode } from '3d-force-graph'

  const canvas = ref<HTMLElement | null>(null)
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

  function loadGraph(presentation: ForceGraph3DInstance, json: string) {
    const graph = JSON.parse(json) as Graph
    addNeighbors(graph)
    presentation.graphData(graph)
  }

  function lookAt(presentation: ForceGraph3DInstance, node: GraphNode) {
    // Aim at node from outside it
    const distance = 70
    const distRatio = 1 + distance / Math.hypot(node.x, node.y, node.z)

    const newPos =
      node.x || node.y || node.z ? { x: node.x * distRatio, y: node.y * distRatio, z: node.z * distRatio } : { x: 0, y: 0, z: distance } // special case if node is in (0,0,0)

    presentation.cameraPosition(newPos, node, 3000)
  }

  function updateHighlight(presentation: ForceGraph3DInstance) {
    // trigger update of highlighted objects in scene
    presentation
      .nodeColor(presentation.nodeColor())
      .linkWidth(presentation.linkWidth())
      .linkDirectionalParticles(presentation.linkDirectionalParticles())
  }

  function highlightNode(presentation: ForceGraph3DInstance, node?: GraphNode) {
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

    updateHighlight(presentation)
  }

  function highlightLink(presentation: ForceGraph3DInstance, link?: GraphLink) {
    highlightNodes.clear()
    highlightLinks.clear()

    if (link) {
      highlightLinks.add(link)
      highlightNodes.add(link.source)
      highlightNodes.add(link.target)
    }

    updateHighlight(presentation)
  }

  function fixNodePosition(node: GraphNode) {
    node.fx = node.x
    node.fy = node.y
    node.fz = node.z
  }

  function getNodeColor(node: GraphNode) {
    return highlightNodes.has(node) ? (node === hoverNode ? 'rgb(255,0,0,1)' : 'rgba(255,160,0,0.8)') : 'rgba(0,255,255,0.6)'
  }

  onMounted(async () => {
    const presentation = new ForceGraph3D(document.getElementById('canvas')!)
      .nodeLabel('id')
      .nodeAutoColorBy('group')
      .nodeColor((node) => getNodeColor(node))
      .linkWidth((link) => (highlightLinks.has(link) ? 4 : 1))
      .linkDirectionalParticles((link) => (highlightLinks.has(link) ? 4 : 0))
      .linkDirectionalParticleWidth(4)
      .showNavInfo(false)
      .linkDirectionalArrowLength(3.5)
      .linkDirectionalArrowRelPos(1)
      .onNodeClick((node: GraphNode) => lookAt(presentation, node))
      .onNodeDragEnd((node: GraphNode) => fixNodePosition(node))
      .onNodeHover((node?: GraphNode, _?: GraphNode) => highlightNode(presentation, node))
      .onLinkHover((link?: GraphLink, _?: GraphLink) => highlightLink(presentation, link))

    window.addEventListener('resize', () => {
      presentation.width(canvas.value!.clientWidth)
      presentation.height(canvas.value!.clientHeight)
    })

    listen<string>('load:json', (event) => loadGraph(presentation, event.payload))
  })
</script>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, Helvetica Neue, PingFang SC, Microsoft YaHei, Source Han Sans SC, Noto Sans CJK SC,
      WenQuanYi Micro Hei, sans-serif;
    font-size: 15px;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  html,
  body,
  #app {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
  }

  .container {
    margin: 0;
    width: 100%;
    height: 100%;
    display: flex;
  }

  #canvas {
    width: 100%;
    height: 100%;
    display: block;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
