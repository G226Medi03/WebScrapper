class WeatherCard extends HTMLElement {
  constructor() {
    super();
  }

  convertWeatherToImgSrc(weather) {
    if (weather === "-") {
      return "-";
    }
    return `
      <img class="w_50 mlr_5" 
           src="./static/component/WeatherGoKr/png/${weather}.png" 
           alt="${weather} icon">
    `;
  }

  getHtml() {
    const date = this.getAttribute("date");
    const day = this.getAttribute("day");
    const description = this.getAttribute("description");
    const weather = this.getAttribute("weather");
    const lowTemp = this.getAttribute("lowTemp") !== "null" ? `${this.getAttribute("lowTemp")}℃` : "-";
    const highTemp = this.getAttribute("highTemp") !== "null" ? `${this.getAttribute("highTemp")}℃` : "-";
    const rainPercent = this.getAttribute("rainPercent") !== "null" ? `${this.getAttribute("rainPercent")}%` : "-";

    const weatherImageHtml = this.convertWeatherToImgSrc(weather);
    const fontColor = day === "토" ? "#0282DD" : day === "일" ? "#DB0015" : "";

    return `
      <div class="WeatherCard wfc sac mac" id="weatherCard" style="width: 110px">
        <div class="WeatherCard_Head mac w_100" style="color: ${fontColor || 'inherit'}">
          <div class="sac fs_16 h_25 fw_600 PRL_15">${date}일(${day})</div>
          <div class="mac sac fs_12 h_20">${description}</div>
        </div>
        <div class="WeatherCard_Body">
          <div class="Row sac h_50 mac">
            ${weatherImageHtml}
          </div>
          <div class="BB Row h_30 sac Temperature">
            <span class="w_50 mlr_5">${lowTemp}</span>
            <span class="w_50 mlr_5">${highTemp}</span>
          </div>
          <div class="Row h_20 sac mac">
            <span class="w_50 mlr_5 mac">${rainPercent}</span>
          </div>
        </div>
      </div>`;
  }

  getCss() {
    return `
      .WeatherCard_Body {
        width: 100%;
        margin-top: 5px;
        margin-bottom: 5px;
      }
      .WeatherCard {
        background-color: white;
        position: relative;
        border-right: 1px solid transparent;
        border-left: 1px solid transparent;
        transition: all 0.1s;
      }
      .WeatherCard::before {
        background-color: transparent;
        height: 2px;
        width: calc(100% + 2px);
        position: relative;
        content: "";
        transition: all 0.1s;
      }
      .WeatherCard>.WeatherCard_Head {
        background-color: #DCE1E9;
        border: 1px solid rgb(197, 207, 219);
        position: relative;
        top: -2px;
        transition: all 0.1s;
      }
      .WeatherCard>.WeatherCard_Body>.BB {
        border-bottom: 1px solid #C9D9EF;
        transition: all 0.1s;
      }
      .WeatherCard_Body>.Temperature>span:first-child {
        color: #0083DD;
      }
      .WeatherCard_Body>.Temperature>span:last-child {
        color: #DB0015;
      }
      .WeatherCard.Hovered, .WeatherCard.Selected {
        border-right: 1px solid rgb(197, 207, 219);
        border-left: 1px solid rgb(197, 207, 219);
        transition: all 0.1s;
      }
      .WeatherCard.Hovered::before, .WeatherCard.Selected::before {
        background-color: #2CABD0;
        height: 2px;
        width: calc(100% + 2px);
        position: relative;
        content: "";
        transition: all 0.1s;
      }
      .WeatherCard.Hovered>.WeatherCard_Head, .WeatherCard.Selected>.WeatherCard_Head {
        background-color: transparent;
        border: 1px solid transparent;
        position: relative;
        top: -2px;
        transition: all 0.1s;
      }
      .WeatherCard.Hovered>.WeatherCard_Body>.BB, .WeatherCard.Selected>.WeatherCard_Body>.BB {
        border-bottom: 1px solid #E4E4E4;
        transition: all 0.1s;
      }
    `;
  }

  addEventListeners(card) {
    card.addEventListener("mouseenter", () => card.classList.add("Hovered"));
    card.addEventListener("mouseleave", () => card.classList.remove("Hovered"));
    card.addEventListener("click", () => card.classList.toggle("Selected"));
  }

  connectedCallback() {
    const template = document.createElement("template");
    template.innerHTML = `
      <style>${this.getCss()}</style>
      ${this.getHtml()}`;

    this.appendChild(template.content.cloneNode(true));
    const weatherCard = this.querySelector("#weatherCard");
    this.addEventListeners(weatherCard);
  }
}

window.customElements.define("weather-card", WeatherCard);

