{% for day in days -%}
  mod day{{ day }};
{% endfor %}
pub mod prelude {
  {% for day in days -%}
    pub use super::day{{ day }}::Day{{ day }}Handler;
  {% endfor -%}
  {{ "// all day handlers" }}
}