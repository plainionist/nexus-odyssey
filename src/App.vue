<template>
  <main class="container">
    <div ref="canvas" id="canvas"></div>
  </main>
</template>

<script setup lang="ts">
  import { onMounted, ref } from 'vue'
  import { listen } from '@tauri-apps/api/event'
  import ForceGraph3D from '3d-force-graph'

  const canvas = ref<HTMLElement | null>(null)

  function loadGraph(data: any, graph: any) {
    const gData = JSON.parse(data)

    const nodeMap = Object.fromEntries(gData.nodes.map((node: any) => [node.id, node]))

    gData.links.forEach((link: any) => {
      const a = nodeMap[link.source]
      const b = nodeMap[link.target]

      console.log(a, b)
      a.neighbors = a.neighbors || []
      b.neighbors = b.neighbors || []
      a.neighbors.push(b)
      b.neighbors.push(a)

      !a.links && (a.links = [])
      !b.links && (b.links = [])
      a.links.push(link)
      b.links.push(link)
    })

    graph.graphData(gData)
  }

  onMounted(async () => {
    const highlightNodes = new Set()
    const highlightLinks = new Set()
    let hoverNode: any = null

    const graph = new ForceGraph3D(document.getElementById('canvas')!)
      .nodeLabel('id')
      .nodeAutoColorBy('group')
      .nodeColor((node) =>
        highlightNodes.has(node) ? (node === hoverNode ? 'rgb(255,0,0,1)' : 'rgba(255,160,0,0.8)') : 'rgba(0,255,255,0.6)'
      )
      .linkWidth((link) => (highlightLinks.has(link) ? 4 : 1))
      .linkDirectionalParticles((link) => (highlightLinks.has(link) ? 4 : 0))
      .linkDirectionalParticleWidth(4)
      .showNavInfo(false)
      .linkDirectionalArrowLength(3.5)
      .linkDirectionalArrowRelPos(1)
      .onNodeClick((node: any) => {
        // Aim at node from outside it
        const distance = 70
        const distRatio = 1 + distance / Math.hypot(node.x, node.y, node.z)

        const newPos =
          node.x || node.y || node.z ? { x: node.x * distRatio, y: node.y * distRatio, z: node.z * distRatio } : { x: 0, y: 0, z: distance } // special case if node is in (0,0,0)

        graph.cameraPosition(
          newPos, // new position
          node, // lookAt ({ x, y, z })
          3000 // ms transition duration
        )
      })
      .onNodeDragEnd((node: any) => {
        node.fx = node.x
        node.fy = node.y
        node.fz = node.z
      })
      .onNodeHover((node: any) => {
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
      })
      .onLinkHover((link: any) => {
        highlightNodes.clear()
        highlightLinks.clear()

        if (link) {
          highlightLinks.add(link)
          highlightNodes.add(link.source)
          highlightNodes.add(link.target)
        }

        updateHighlight()
      })

    function updateHighlight() {
      // trigger update of highlighted objects in scene
      graph.nodeColor(graph.nodeColor()).linkWidth(graph.linkWidth()).linkDirectionalParticles(graph.linkDirectionalParticles())
    }

    window.addEventListener('resize', () => {
      graph.width(canvas.value!.clientWidth)
      graph.height(canvas.value!.clientHeight)
    })

    listen('load:json', (e: any) => loadGraph(e.payload, graph))
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
