declare module '3d-force-graph' {
  export interface Point3d {
    x: number
    y: number
    z: number
  }

  export interface GraphNode extends Point3d {
    id: string | number
    [key: string]: any // Allow additional properties
  }

  export interface GraphLink {
    source: string
    target: string
    [key: string]: any
  }

  export interface GraphMeta {
    semantics?: string
  }

  export interface Graph {
    meta?: GraphMeta
    nodes: GraphNode[]
    links: GraphLink[]
  }

  export interface ForceGraph3DInstance {
    new (element: HTMLElement): ForceGraph3DInstance // Constructor accepting an HTMLElement

    nodes: GraphNode[]
    links: GraphLink[]
    graphData(data?: Graph): ForceGraph3DInstance

    width(value: Number): ForceGraph3DInstance
    height(value: Number): ForceGraph3DInstance

    nodeColor(): string
    nodeColor(color: string | ((node: GraphNode) => string)): ForceGraph3DInstance
    nodeLabel(label: string | ((node: GraphNode) => string)): ForceGraph3DInstance
    nodeVal(val: number | ((node: GraphNode) => number)): ForceGraph3DInstance
    nodeOpacity(opacity: number): ForceGraph3DInstance
    nodeRelSize(size: number): ForceGraph3DInstance
    nodeResolution(resolution: number): ForceGraph3DInstance
    nodeThreeObject(callback: (node: GraphNode) => THREE.Object3D): ForceGraph3DInstance
    nodeAutoColorBy(colorBy: string | ((node: GraphNode) => string)): ForceGraph3DInstance

    linkColor(color: string | ((link: GraphLink) => string)): ForceGraph3DInstance
    linkWidth(): number
    linkWidth(width: number | ((link: GraphLink) => number)): ForceGraph3DInstance
    linkResolution(resolution: number): ForceGraph3DInstance
    linkDirectionalParticles(): number
    linkDirectionalParticles(numParticles: number | ((link: GraphLink) => number)): ForceGraph3DInstance
    linkDirectionalParticleSpeed(speed: number | ((link: GraphLink) => number)): ForceGraph3DInstance
    linkDirectionalParticleWidth(width: number | ((link: GraphLink) => number)): ForceGraph3DInstance
    linkDirectionalArrowLength(width: number | String | ((link: GraphLink) => number)): ForceGraph3DInstance
    linkDirectionalArrowRelPos(width: number | String | ((link: GraphLink) => number)): ForceGraph3DInstance
    linkThreeObject(callback: (link: GraphLink) => THREE.Object3D): ForceGraph3DInstance

    d3Force(forceName: string, forceFn?: d3.ForceSimulation<any, any>): ForceGraph3DInstance | d3.ForceSimulation<any, any>
    d3Alpha(alpha?: number): ForceGraph3DInstance | number
    d3AlphaMin(alphaMin?: number): ForceGraph3DInstance | number
    d3AlphaDecay(alphaDecay?: number): ForceGraph3DInstance | number
    d3VelocityDecay(velocityDecay?: number): ForceGraph3DInstance | number

    cameraPosition(position: Point3d, lookAt?: Point3d, transitionMs?: number): ForceGraph3DInstance
    backgroundColor(color: string): ForceGraph3DInstance
    showNavInfo(show: boolean): ForceGraph3DInstance

    onNodeClick(callback: (node: GraphNode, event: MouseEvent) => void): ForceGraph3DInstance
    onNodeRightClick(callback: (node: GraphNode, event: MouseEvent) => void): ForceGraph3DInstance
    onNodeHover(callback: (node?: GraphNode, prevNode?: GraphNode) => void): ForceGraph3DInstance
    onNodeDragEnd(callback: (link: GraphNode, translate: Point3d) => void): ForceGraph3DInstance
    onLinkClick(callback: (link: GraphLink, event: MouseEvent) => void): ForceGraph3DInstance
    onLinkHover(callback: (link?: GraphLink, prevLink?: GraphLink) => void): ForceGraph3DInstance

    refresh(): ForceGraph3DInstance
  }

  const ForceGraph3D: {
    new (element: HTMLElement): ForceGraph3DInstance
  }

  export default ForceGraph3D
}
