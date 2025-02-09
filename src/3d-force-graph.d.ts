declare module '3d-force-graph' {
  export interface NodeObject {
    id: string | number
    [key: string]: any // Allow additional properties
  }

  export interface LinkObject {
    source: string | number | NodeObject
    target: string | number | NodeObject
    [key: string]: any
  }

  export interface GraphData {
    nodes: NodeObject[]
    links: LinkObject[]
  }

  export interface Translate {
    x: Number
    y: Number
    z: Number
  }

  export interface ForceGraph3DInstance {
    new (element: HTMLElement): ForceGraph3DInstance // Constructor accepting an HTMLElement

    graphData(data?: GraphData): ForceGraph3DInstance

    width(value: Number): ForceGraph3DInstance
    height(value: Number): ForceGraph3DInstance

    nodeColor(): string
    nodeColor(color: string | ((node: NodeObject) => string)): ForceGraph3DInstance
    nodeLabel(label: string | ((node: NodeObject) => string)): ForceGraph3DInstance
    nodeVal(val: number | ((node: NodeObject) => number)): ForceGraph3DInstance
    nodeOpacity(opacity: number): ForceGraph3DInstance
    nodeRelSize(size: number): ForceGraph3DInstance
    nodeResolution(resolution: number): ForceGraph3DInstance
    nodeThreeObject(callback: (node: NodeObject) => THREE.Object3D): ForceGraph3DInstance
    nodeAutoColorBy(colorBy: string | ((node: NodeObject) => string)): ForceGraph3DInstance

    linkColor(color: string | ((link: LinkObject) => string)): ForceGraph3DInstance
    linkWidth(): number
    linkWidth(width: number | ((link: LinkObject) => number)): ForceGraph3DInstance
    linkResolution(resolution: number): ForceGraph3DInstance
    linkDirectionalParticles(): number
    linkDirectionalParticles(numParticles: number | ((link: LinkObject) => number)): ForceGraph3DInstance
    linkDirectionalParticleSpeed(speed: number | ((link: LinkObject) => number)): ForceGraph3DInstance
    linkDirectionalParticleWidth(width: number | ((link: LinkObject) => number)): ForceGraph3DInstance
    linkDirectionalArrowLength(width: number | String | ((link: LinkObject) => number)): ForceGraph3DInstance
    linkDirectionalArrowRelPos(width: number | String | ((link: LinkObject) => number)): ForceGraph3DInstance
    linkThreeObject(callback: (link: LinkObject) => THREE.Object3D): ForceGraph3DInstance

    d3Force(forceName: string, forceFn?: d3.ForceSimulation<any, any>): ForceGraph3DInstance | d3.ForceSimulation<any, any>
    d3Alpha(alpha?: number): ForceGraph3DInstance | number
    d3AlphaMin(alphaMin?: number): ForceGraph3DInstance | number
    d3AlphaDecay(alphaDecay?: number): ForceGraph3DInstance | number
    d3VelocityDecay(velocityDecay?: number): ForceGraph3DInstance | number

    cameraPosition(
      position: { x: number; y: number; z: number },
      lookAt?: { x: number; y: number; z: number },
      transitionMs?: number
    ): ForceGraph3DInstance
    backgroundColor(color: string): ForceGraph3DInstance
    showNavInfo(show: boolean): ForceGraph3DInstance

    onNodeClick(callback: (node: NodeObject, event: MouseEvent) => void): ForceGraph3DInstance
    onNodeHover(callback: (node?: NodeObject, prevNode?: NodeObject) => void): ForceGraph3DInstance
    onNodeDragEnd(callback: (link: NodeObject, translate: Translate) => void): ForceGraph3DInstance
    onLinkClick(callback: (link: LinkObject, event: MouseEvent) => void): ForceGraph3DInstance
    onLinkHover(callback: (link?: LinkObject, prevLink?: LinkObject) => void): ForceGraph3DInstance

    refresh(): ForceGraph3DInstance
  }

  const ForceGraph3D: {
    new (element: HTMLElement): ForceGraph3DInstance
  }

  export default ForceGraph3D
}
