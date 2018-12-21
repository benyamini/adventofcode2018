export function Activity(date, activity) {
  this.date = date;
  this.activity = activity;
  this.display = displayActivity;
}

export function displayActivity() {
  console.log(`date: ${this.date}\nactivity: ${this.activity}\n`);
}

export function Guard(id) {
  this.id = id;
  this.shifts = [];
  this.display = displayGuard;
}

export function displayGuard() {
  console.log(`guard id: ${this.id}`);
}

export function Shift() {
  this.sleeptime = [];
  this.display = displayTimes;
}

export function displayTimes() {
  console.log(`time: ${this.times}`);
}

export function containsGuard(id, list) {
    for (i = 0; i < list.length; i++) {
        if (list[i].id === id) {
            return true;
        }
    }

    return false;
}