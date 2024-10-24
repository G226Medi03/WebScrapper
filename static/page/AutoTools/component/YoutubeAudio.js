class YoutubeAudio extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {
        return `<div class="sas">
                    <div class="fs_32">
                        Youtube Audio Download
                    </div>
                    <div class="fs_24 Row mas sac">
                        <div style="width: 10px;">
                        </div>
                        <input placeholder="  link url" class="br_5 h_35 fs_20" style="width: 800px;" type="text">

                        <button class="NoEnter fs_20 ptb_5 plr_15 br_5">Get Audio</button>
                    </div>
                </div>`;
    }
    getCss() {

        const style = ``;

        return style;
    }

    async search() {
        let input = this.querySelector("input");
        window.location.href=`/youtube/audio?url=${input.value}`
    }

    runJs() {
        let button = this.querySelector("button");
        button.addEventListener("click", async (event) => {
            await this.search();  
        })


    }

    connectedCallback() {
        const template = document.createElement("template");
        template.innerHTML = `
        <style>
            ${this.getCss()}
        </style>
        ${this.getHtml()}`;

        this.appendChild(template.content.cloneNode(true));
        this.runJs();
    }
}

window.customElements.define("youtube-audio", YoutubeAudio);
