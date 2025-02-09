<template>
  <main class="container">
    <div ref="canvas" id="canvas"></div>
  </main>
</template>

<script setup lang="ts">
  import { onMounted, ref } from 'vue'
  import { listen } from '@tauri-apps/api/event'
  import ForceGraph3D, { ForceGraph3DInstance, Graph, GraphLink, GraphNode } from '3d-force-graph'
  import { useHighlight } from './composables/useHighlight'

  const canvas = ref<HTMLElement | null>(null)

  function lookAt(presentation: ForceGraph3DInstance, node: GraphNode) {
    // Aim at node from outside it
    const distance = 70
    const distRatio = 1 + distance / Math.hypot(node.x, node.y, node.z)

    const newPos =
      node.x || node.y || node.z ? { x: node.x * distRatio, y: node.y * distRatio, z: node.z * distRatio } : { x: 0, y: 0, z: distance } // special case if node is in (0,0,0)

    presentation.cameraPosition(newPos, node, 3000)
  }

  function fixNodePosition(node: GraphNode) {
    node.fx = node.x
    node.fy = node.y
    node.fz = node.z
  }

  onMounted(async () => {
    const presentation = new ForceGraph3D(document.getElementById('canvas')!)

    const { addNeighbors, highlightNode, highlightLink, getNodeColor, isHighlighted } = useHighlight(presentation)

    presentation
      .nodeLabel('id')
      .nodeAutoColorBy('group')
      .nodeColor((node) => getNodeColor(node))
      .linkWidth((link) => (isHighlighted(link) ? 4 : 1))
      .linkDirectionalParticles((link) => (isHighlighted(link) ? 4 : 0))
      .linkDirectionalParticleWidth(4)
      .showNavInfo(false)
      .linkDirectionalArrowLength(3.5)
      .linkDirectionalArrowRelPos(1)
      .onNodeClick((node: GraphNode) => lookAt(presentation, node))
      .onNodeDragEnd((node: GraphNode) => fixNodePosition(node))
      .onNodeHover((node?: GraphNode, _?: GraphNode) => highlightNode(node))
      .onLinkHover((link?: GraphLink, _?: GraphLink) => highlightLink(link))

    window.addEventListener('resize', () => {
      presentation.width(canvas.value!.clientWidth)
      presentation.height(canvas.value!.clientHeight)
    })

    listen<string>('load:json', (event) => {
      const graph = JSON.parse(event.payload) as Graph
      addNeighbors(graph)
      presentation.graphData(graph)
    })
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
