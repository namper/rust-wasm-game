import { Game } from 'rust-web-game'
import config from './config'
import { View } from './view'

export class GameController{
    constructor(){
        this.setUp()
        this.view = new View(
            this.game.width,
            this.game.height,
            this.render.bind(this)
        )
    }

    setUp(){
        this.game = new Game(
            config.WIDTH,
            config.HEIGHT
        )

        console.log(this.game)
    }

    render(){
        this.view.render(this.game.pos)
    }

    setup_key_events(){
        document.addEventListener("keydown", (ev) => {
            this.game.key_down(ev);
        });

        document.addEventListener("keyup", (ev) => {
            this.game.key_up(ev);
        });
    }

    tick() {
        const lastUpdate = Date.now()
        if (this.lastUpdate) {
          this.game.process(lastUpdate - this.lastUpdate)
        }
        this.lastUpdate = lastUpdate
        this.render()

      }
    
      run() {
        this.setup_key_events();
        setInterval(this.tick.bind(this), 1000 / config.FPS)
      }
}
