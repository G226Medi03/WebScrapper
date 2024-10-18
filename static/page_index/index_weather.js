import HttpClient from "/static/HttpClient.js";

async function initWeatherCards() {
  let httpClient = new HttpClient();

  let jData = "";
  httpClient.getUrl("./weathers", (msg) => {
    jData = JSON.parse(msg);

    let htmlElem = document.querySelector(".WeatherCardContainer");

    let sHtml = "";
    jData.forEach((jObj) => {
      sHtml += `<weather-card 
          date="${jObj["date"]}"
          day="${jObj["day"]}" 
          description="${jObj["description"]}"
          weather="${jObj["weather"]}" 
          lowTemp="${jObj["lowTemp"]}"
          highTemp="${jObj["highTemp"]}"
          rainPercent="${jObj["rainPercent"]}"
          >
          </weather-card>`;
    });
    htmlElem.innerHTML = sHtml;
  });
}


export default async function updateWeatherCards() {
  //style 조정
  document.addEventListener("DOMContentLoaded", (e) => {
    initWeatherCards();

    let cards = [];
    document.querySelectorAll("weather-card").forEach((webComponent) => {
      let card = webComponent.querySelector(".WeatherCard");
      cards.push(card);
    });
    cards.forEach((card) => {
      card.addEventListener("click", (e) => {
        cards.forEach((_card) => {
          if (_card != card) {
            _card.classList.remove("Selected");
          }
        });
      });
    });
  });

}
