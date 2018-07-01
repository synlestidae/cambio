export class CalendarDate {
    public day: number = 1;
    public month: number = 1;
    public year: number = 2000;
    private date: Date = new Date(2000, 0, 1);

    public changeDay(positive: boolean) {
        let day = this.day;
        if (isNaN(day)) {
            day = 1;
        }
        let newDay = this.date.getDate() + (positive? 1 : -1);
        this.date.setDate(newDay);
        this.day = newDay;
    }

    public changeMonth(positive: boolean) {
        let month = this.month;
        let newMonth = this.date.getMonth() + (positive? 1 : -1);
        this.date.setMonth(newMonth);
        this.month = newMonth;
    }

    public changeYear(positive: boolean) {
        let year = this.year;
        let newYear = this.date.getFullYear() + (positive? 1 : -1);
        this.date.setFullYear(newYear);
        this.year = newYear;
    }

    public getDate(): Date {
        return new Date(this.date);
    }

    public getDateString(): string {
        return `${pad2(this.year)}-${pad2(this.month)}-${pad2(this.day)}`;
    }

    public parseDateString(dateStr: string) {
        let rx = /(\d{1,2})\/(\d{1,2})\/(\d{1,2})/;
        // meh
    }
}

function pad2(num: number) {
    let text = num.toString();
    if (text.length < 2) {
        return '0' + text;
    }
    return text;
}
