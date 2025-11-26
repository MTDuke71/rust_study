# 🎯 Daily Focus Dashboard

*Automated daily learning focus using Dataview*

---

## 📅 Current Date

```dataview
TABLE WITHOUT ID
  dateformat(date(today), "EEEE, MMMM d, yyyy") as "Today"
```

---

## 📚 Today's Learning Plan

```dataview
LIST
WHERE file.name = "MONTHLY_CALENDAR"
FLATTEN file.content as content
WHERE contains(content, dateformat(date(today), "MMMM d"))
```

---

## 🔗 Quick Navigation

- **[[MONTHLY_CALENDAR]]** - Full monthly calendar
- **[[zettel-index]]** - Main knowledge hub
- **[[Daily Study MOC]]** - Current week overview
- **[[Missions Overview]]** - Active missions

---

*Tags: #dashboard #auto-generated #daily-focus*
*Links: [[MONTHLY_CALENDAR]] | [[zettel-index]] | [[Daily Study MOC]] | [[Missions Overview]]*
