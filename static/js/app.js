const weatherForm = document.querySelector('form')
const search = document.querySelector('input')
const messageOne = document.querySelector('.messageOne')
const city = document.querySelector('.city')
const region = document.querySelector('.region')
const country = document.querySelector('.country')
const time = document.querySelector('.time')
const temperature = document.querySelector('.temp')
const feels_like = document.querySelector('.feels-like')
const description = document.querySelector('.description')
const image = document.querySelector('.image')
const precipitation = document.querySelector('.precipitation')
const humidity = document.querySelector('.humidity')
const wind = document.querySelector('.wind')

const fetchAddress = (address) => {
    fetch(`/weather?address=${address}`)
        .then((response) => {
            response.json()
            .then((data) => {
                if (data.error) {
                    messageOne.textContent = data.error
                    messageOne.classList.add('error')
                } else {
                  // console.log(data)
                    messageOne.textContent = ''

                    city.textContent = data.location.name
                    country.textContent = data.location.country
                    region.textContent = data.location.region

                    let current_time = parseTime(data.location.localtime)
                    time.textContent = convert24To12(`${current_time}`)

                    temperature.textContent = data.current.temperature
                    feels_like.textContent = data.current.feelslike

                    image.src = data.current.weather_icons[0]

                    description.textContent = data.current.weather_descriptions[0]
                    precipitation.textContent = `Precipitation: ${data.current.precip}`
                    humidity.textContent = `Humidity: ${data.current.humidity}`
                    wind.textContent = `Wind: ${data.current.wind_speed} ${data.current.wind_dir}`
                }
            })
        })
}

weatherForm.addEventListener('submit', (e) => {
    e.preventDefault()
    const location = search.value
    messageOne.classList.remove('error')
    messageOne.textContent = 'Searching...'
    fetchAddress(location)
});

function parseTime(time) {
  // get the time from the weather data - data.location.localtime
  let [date, rawTime] = time.split(" ")
  return rawTime

}

function convert24To12(time) {
  // turn the 24hr time to 12hr time
  let [hours, minutes] = time.split(":")
  let pm = false
  hours = parseInt(hours, 10)
  if (hours >= 12) {
    hours -= 12
    pm = true
  }
  if (hours === 0) {
    hours = 12
  }
  const formattedHours = hours.toString().padStart(2, "0")
  const formattedMinutes = minutes.toString().padStart(2, "0")
  return `${formattedHours}:${formattedMinutes} ${pm ? "PM" : "AM"}`
}