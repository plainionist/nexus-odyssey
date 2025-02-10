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
  import SpriteText from 'three-spritetext'
  import { invoke } from '@tauri-apps/api/core'

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

  onMounted(async () => {
    const presentation = new ForceGraph3D(document.getElementById('canvas')!)

    const { addNeighbors, highlightNode, highlightLink, isHighlighted, isHovered } = useHighlight(presentation)

    function getBorderColor(node: GraphNode): string {
      return isHighlighted(node) ? (isHovered(node) ? 'red' : 'orange') : 'darkgray'
    }

    const nodeObjects = new Map<GraphNode, SpriteText>()

    presentation
      .nodeLabel('id')
      .linkWidth((link) => (isHighlighted(link) ? 4 : 3))
      .linkDirectionalParticles((link) => (isHighlighted(link) ? 4 : 0))
      .linkDirectionalParticleWidth(4)
      .showNavInfo(false)
      // .linkDirectionalArrowLength(3.5)
      // .linkDirectionalArrowRelPos(1)
      .onNodeClick((node: GraphNode) => lookAt(presentation, node))
      .onNodeDragEnd((node: GraphNode) => fixNodePosition(node))
      .onNodeHover((node?: GraphNode) => {
        highlightNode(node)

        nodeObjects.forEach((obj, n) => {
          obj.borderColor = getBorderColor(n)
        })
      })
      .onLinkHover((link?: GraphLink, _?: GraphLink) => highlightLink(link))
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
    presentation.d3Force('charge').strength(-120)

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
      background-color: #1e1e1e;
    }
  }
</style>
