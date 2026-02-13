# Zenith UI Widgets API Reference

## Overview
Zenith provides a declarative UI system using JSON-like objects. All widgets are defined as objects with a `"type"` field.

---

## Layout Widgets

### Column
Arranges children vertically (top to bottom).

```zenith
{
    "type": "Column",
    "children": [
        { "type": "Text", "text": "First" },
        { "type": "Text", "text": "Second" },
        { "type": "Text", "text": "Third" }
    ]
}
```

**Properties**:
- `children`: Array of child widgets

---

### Row
Arranges children horizontally (left to right).

```zenith
{
    "type": "Row",
    "children": [
        { "type": "Button", "label": "Left" },
        { "type": "Button", "label": "Middle" },
        { "type": "Button", "label": "Right" }
    ]
}
```

**Properties**:
- `children`: Array of child widgets

---

### Center
Centers a single child widget.

```zenith
{
    "type": "Center",
    "child": {
        "type": "Text",
        "text": "Centered Text"
    }
}
```

**Properties**:
- `child`: Single child widget

---

### Container
A wrapper widget with optional styling.

```zenith
{
    "type": "Container",
    "child": {
        "type": "Column",
        "children": [...]
    }
}
```

**Properties**:
- `child`: Single child widget

---

## Display Widgets

### Text
Displays text.

```zenith
{
    "type": "Text",
    "text": "Hello, Zenith!"
}
```

**Properties**:
- `text`: String to display

**Example with Variables**:
```zenith
var count = 42
var ui = {
    "type": "Text",
    "text": "Count: " + count
}
```

---

## Input Widgets

### Button
A clickable button.

```zenith
{
    "type": "Button",
    "label": "Click Me",
    "on_click": "my_button"
}
```

**Properties**:
- `label`: Button text (default: "Button")
- `on_click`: Event ID when clicked

**Event Handling**:
```zenith
var event = wait_for_event()
if (event["type"] == "click" && event["id"] == "my_button") {
    println("Button was clicked!")
}
```

---

### TextField
Single-line text input.

```zenith
{
    "type": "TextField",
    "value": "Initial text",
    "on_change": "my_input"
}
```

**Properties**:
- `value`: Current text value
- `on_change`: Event ID when text changes

**Event Handling**:
```zenith
var event = wait_for_event()
if (event["type"] == "input" && event["id"] == "my_input") {
    var new_text = event["value"]
    println("User typed: " + new_text)
}
```

---

## Complete UI Example

### Simple Form
```zenith
func build_ui(state) {
    return {
        "type": "Center",
        "child": {
            "type": "Container",
            "child": {
                "type": "Column",
                "children": [
                    { "type": "Text", "text": "Login Form" },
                    
                    {
                        "type": "TextField",
                        "value": state["username"],
                        "on_change": "username_input"
                    },
                    
                    {
                        "type": "TextField",
                        "value": state["password"],
                        "on_change": "password_input"
                    },
                    
                    {
                        "type": "Row",
                        "children": [
                            {
                                "type": "Button",
                                "label": "Login",
                                "on_click": "login_btn"
                            },
                            {
                                "type": "Button",
                                "label": "Cancel",
                                "on_click": "cancel_btn"
                            }
                        ]
                    }
                ]
            }
        }
    }
}
```

---

## Dynamic UI Building

### Building Lists
```zenith
func build_todo_list(items) {
    var list_widgets = []
    
    for i in 0..len(items) {
        var item_widget = {
            "type": "Row",
            "children": [
                { "type": "Text", "text": items[i] },
                {
                    "type": "Button",
                    "label": "Delete",
                    "on_click": "delete_" + i
                }
            ]
        }
        list_widgets = push(list_widgets, item_widget)
    }
    
    return {
        "type": "Column",
        "children": list_widgets
    }
}
```

---

## Event Types

### Click Event
```zenith
{
    "type": "click",
    "id": "button_id"
}
```

Triggered by: `Button`

### Input Event
```zenith
{
    "type": "input",
    "id": "field_id",
    "value": "user text"
}
```

Triggered by: `TextField`

---

## Reactive UI Pattern

### 1. Create State
```zenith
var state = create_state({
    "count": 0,
    "text": "Hello"
})
```

### 2. Build UI Function
```zenith
func build_ui(state_value) {
    return {
        "type": "Column",
        "children": [
            { "type": "Text", "text": "Count: " + state_value["count"] },
            { "type": "Button", "label": "+1", "on_click": "increment" }
        ]
    }
}
```

### 3. Event Loop
```zenith
while (true) {
    var current = get_state(state)
    render_ui(build_ui(current))
    
    var event = wait_for_event()
    
    if (event["type"] == "click" && event["id"] == "increment") {
        current["count"] = current["count"] + 1
        set_state(state, current)
    }
}
```

---

## Widget Tree Structure

```
Center (centers content)
└─ Container (wrapper)
   └─ Column (vertical layout)
      ├─ Text (display)
      ├─ Row (horizontal layout)
      │  ├─ Button
      │  └─ Button
      └─ TextField (input)
```

---

## Best Practices

### 1. Component Functions
Break down complex UIs into reusable functions:

```zenith
func build_card(title, content) {
    return {
        "type": "Container",
        "child": {
            "type": "Column",
            "children": [
                { "type": "Text", "text": title },
                { "type": "Text", "text": content }
            ]
        }
    }
}

var ui = {
    "type": "Column",
    "children": [
        build_card("Card 1", "Content 1"),
        build_card("Card 2", "Content 2")
    ]
}
```

### 2. State Management
Keep UI state in a single object:

```zenith
var state = {
    "user": {
        "name": "Alice",
        "logged_in": false
    },
    "ui": {
        "current_page": "home",
        "theme": "dark"
    }
}
```

### 3. Event Naming
Use descriptive event IDs:

```zenith
// Good
"on_click": "login_button"
"on_click": "delete_item_5"

// Bad
"on_click": "btn1"
"on_click": "click"
```

---

## Widget Summary Table

| Widget | Type | Children | Purpose |
|--------|------|----------|---------|
| Column | Layout | Multiple | Vertical stack |
| Row | Layout | Multiple | Horizontal stack |
| Center | Layout | Single | Center content |
| Container | Layout | Single | Wrapper/padding |
| Text | Display | None | Show text |
| Button | Input | None | Clickable button |
| TextField | Input | None | Text input |

---

## Future Widgets (Roadmap)

- Image: Display images
- Slider: Numeric input
- Checkbox: Boolean toggle
- Radio: Single choice
- ScrollArea: Scrollable container
- TabView: Tabbed interface
- ProgressBar: Loading indicator

---

**See Also**:
- [StdLib API Reference](stdlib.md)
- [Tutorial: First GUI](../tutorial/11_first_gui.md)
- [Examples](../../examples/)
