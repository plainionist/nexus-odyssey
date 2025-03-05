<template>
  <main class="container">
    <input v-if="mode === 'markdown'" type="text" v-model="searchQuery" @keyup.enter="searchNode" placeholder="Search ..." />
    <div ref="canvas" id="canvas"></div>
  </main>
</template>

<script setup lang="ts">
  import { onMounted, ref } from 'vue'
  import { listen } from '@tauri-apps/api/event'
  import ForceGraph3D, { ForceGraph3DInstance, Graph, GraphLink, GraphNode } from '3d-force-graph'
  import { useHighlight } from './composables/useHighlight'
  import SpriteText from 'three-spritetext'
  import { invoke } from '@tauri-apps/api/core'

  const canvas = ref<HTMLElement | null>(null)
  const mode = ref('')
  const searchQuery = ref('')
  const presentation = ref<ForceGraph3DInstance | null>(null)

  const { addNeighbors, highlightNode, highlightLink, isHighlighted, isHovered } = useHighlight(presentation)

  function searchNode() {
    const node = presentation.value!.graphData().nodes.find((n: GraphNode) => n.title.includes(searchQuery.value))

    if (node) {
      lookAt(presentation.value!, node)
    } else {
      console.warn('Node not found!')
    }
  }

  function lookAt(presentation: ForceGraph3DInstance, node: GraphNode) {
    // Aim at node from outside it
    const distance = 170
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

  function getNodeColor(node: GraphNode): string {
    switch (node.kind) {
      case 'file':
        return 'darkblue'
      case 'topic':
        return 'green'
      default:
        return 'rgba(255,255,255,0.6)'
    }
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text).catch((err) => {
      console.error('Failed to copy text', err)
    })
  }

  function openFileInVSCode(filePath: string) {
    invoke('open_in_vscode', { filePath }).catch(console.error)
  }

  function buildPresentation(graph: Graph) {
    mode.value = graph.meta?.semantics || ''

    presentation.value = new ForceGraph3D(document.getElementById('canvas')!)

    function getBorderColor(node: GraphNode): string {
      return isHighlighted(node) ? (isHovered(node) ? 'red' : 'orange') : 'darkgray'
    }

    const nodeObjects = new Map<GraphNode, SpriteText>()

    presentation.value
      //.nodeLabel('id')
      .linkDirectionalParticles((link: GraphLink) => (isHighlighted(link) ? 4 : 0))
      .linkDirectionalParticleWidth(4)
      .backgroundColor('#1e1e1e00')
      .showNavInfo(false)
      // .linkDirectionalArrowLength(3.5)
      // .linkDirectionalArrowRelPos(1)
      .onNodeClick((node: GraphNode) => lookAt(presentation.value!, node))
      .onNodeDragEnd((node: GraphNode) => fixNodePosition(node))
      .onLinkHover((link?: GraphLink, _?: GraphLink) => highlightLink(link))

    if (mode.value === 'markdown') {
      presentation.value
        .linkWidth((link: GraphLink) => (isHighlighted(link) ? 4 : 3))
        .onNodeHover((node?: GraphNode) => {
          highlightNode(node)

          nodeObjects.forEach((obj, n) => {
            obj.borderColor = getBorderColor(n)
          })
        })
        .nodeThreeObject((node: GraphNode) => {
          const sprite = new SpriteText(node.title)
          sprite.color = 'white'
          sprite.backgroundColor = getNodeColor(node)
          sprite.textHeight = 8
          sprite.padding = 4
          sprite.borderRadius = 4
          sprite.fontFace = 'Arial'
          sprite.borderWidth = 1
          sprite.borderColor = getBorderColor(node)
          nodeObjects.set(node, sprite)
          return sprite
        })
        .onNodeRightClick((node: GraphNode, event: MouseEvent) => {
          if (!node || !event) return

          if (node.kind === 'file' && event.ctrlKey && event.shiftKey) {
            openFileInVSCode(node.id.toString())
          } else if (event.ctrlKey) {
            copyToClipboard(node.id.toString())
          }
        })

      // Spread nodes a little wider
      presentation.value.d3Force('charge').strength(-120)
    } else {
      presentation.value
        .linkWidth((link: GraphLink) => (isHighlighted(link) ? 4 : 1))
        .nodeAutoColorBy('group')
        .onNodeHover((node?: GraphNode) => highlightNode(node))
    }

    presentation.value.graphData(graph)
    setTimeout(requestFullscreen, 1000)
  }

  function requestFullscreen() {
    if (presentation.value) {
      presentation.value.width(canvas.value!.clientWidth)
      presentation.value.height(canvas.value!.clientHeight)
    }
  }
  onMounted(async () => {
    window.addEventListener('resize', requestFullscreen)

    listen<string>('load:json', (event) => {
      const graph = JSON.parse(event.payload) as Graph
      addNeighbors(graph)
      buildPresentation(graph)
    })
  })
</script>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, Helvetica Neue, PingFang SC, Microsoft YaHei, Source Han Sans SC, Noto Sans CJK SC,
      WenQuanYi Micro Hei, sans-serif;
    font-size: 15px;

    color: #1e1e1e;
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
    display: flex;
    display: flex;
    flex-direction: column; /* Stack items vertically */
    width: 100%;
    height: 100vh; /* Full screen height */
    background-color: black;
  }

  input {
    padding: 10px;
    font-size: 16px;
    width: 100%;
    box-sizing: border-box;
    text-align: left;
    padding: 4px;
  }

  #canvas {
    flex-grow: 1; /* Takes up remaining space */
    width: 100%;
    height: 100%;
    display: block;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #1e1e1e;
    }
  }
</style>
