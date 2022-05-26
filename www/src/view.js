const getRange = length => [...Array(length).keys()]


export class View {
    constructor(gameWidth, gameHeight, onViewChange = () => {}) {
      this.gameWidth = gameWidth
      this.gameHeight = gameHeight
      this.container = document.getElementById('container')
      this.onViewChange = onViewChange
      this.setUp()

      this.pos = [0, 0];
      this.speed = 10/100;
    
      window.addEventListener('resize', () => {
        const [child] = this.container.children
        if (child) {
          this.container.removeChild(child)
        }
        this.setUp()
        this.onViewChange()
      })
    }
  
    setUp() {
        const { width, height } = this.container.getBoundingClientRect()
        this.unitOnScreen = Math.min(
          width / this.gameWidth,
          height / this.gameHeight
        )
        this.projectDistance = distance => distance * this.unitOnScreen
        this.projectPosition = position => position.scale_by(this.unitOnScreen)
        const canvas = document.createElement('canvas')
        this.container.appendChild(canvas)
        this.context = canvas.getContext('2d')
        canvas.setAttribute('width', this.projectDistance(this.gameWidth))
        canvas.setAttribute('height', this.projectDistance(this.gameHeight))
    }


    render() {
        this.context.clearRect(
          0,
          0,
          this.context.canvas.width,
          this.context.canvas.height
        )

        this.context.globalAlpha = 0.2
        this.context.fillStyle = 'black'
        getRange(this.gameWidth).forEach(column =>
          getRange(this.gameHeight)
          .filter(row => (column + row) % 2 === 1)
          .forEach(row =>
            this.context.fillRect(
              column * this.unitOnScreen,
              row * this.unitOnScreen,
              this.unitOnScreen,
              this.unitOnScreen
            )
          )
        )
        this.context.globalAlpha = 1

        // moving rectangle
        this.pos[0] = this.pos[0] + this.speed;
        if (this.pos[0] > this.gameWidth){
            this.pos[0] = -1;

        }

        this.context.fillStyle = "#FF0000";
        this.context.fillRect(
            this.pos[0] * this.unitOnScreen,
            this.pos[1],
            this.unitOnScreen,
            this.unitOnScreen
        )
      }
  }
