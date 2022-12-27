const weatherForm = document.querySelector('form');
const search = document.querySelector('input');
const messageOne = document.querySelector('.messageOne');
const messageTwo = document.querySelector('.messageTwo');
const messageThree = document.querySelector('.messageThree');


const fetchAddress = (address) => {
    fetch(`/weather?address=${address}`)
        .then((response) => {
            response.json()
            .then((data) => {
                if (data.error) {
                    messageOne.textContent = data.error;
                    messageOne.classList.add('error');
                } else {
                    messageOne.textContent = data.location.name;
                    messageTwo.textContent = data.current.temperature;
                    let time = parseTime(data.location.localtime)
                    messageThree.textContent = convert24To12(`${time}`);
                }
            });
        });
}

weatherForm.addEventListener('submit', (e) => {
    e.preventDefault();
    const location = search.value;
    messageOne.classList.remove('error');
    messageOne.textContent = 'Searching...';
    fetchAddress(location);
});

function parseTime(time) {
  // get the time from the weather data - data.location.localtime
  let [date, rawTime] = time.split(" ");
  return rawTime

}

function convert24To12(time) {
  // turn the 24hr time to 12hr time
  let [hours, minutes] = time.split(":");
  let pm = false;
  hours = parseInt(hours, 10);
  if (hours >= 12) {
    hours -= 12;
    pm = true;
  }
  if (hours === 0) {
    hours = 12;
  }
  const formattedHours = hours.toString().padStart(2, "0");
  const formattedMinutes = minutes.toString().padStart(2, "0");
  return `${formattedHours}:${formattedMinutes} ${pm ? "PM" : "AM"}`;
}