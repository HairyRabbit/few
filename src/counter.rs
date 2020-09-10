use dodrio::{bumpalo, Node, Render, RenderContext};
use typed_html::dodrio;

// mod counter_scss;
use crate::counter_scss as style;

/// A counter that can be incremented and decremented!
pub struct Counter {
  count: isize,
}

impl Counter {
  /// Construct a new, zeroed counter.
  pub fn new() -> Counter {
      Counter { count: 0 }
  }

  /// Increment this counter's count.
  fn increment(&mut self) {
      self.count += 1;
  }

  /// Decrement this counter's count.
  fn decrement(&mut self) {
      self.count -= 1;
  }
}

// The `Render` implementation for `Counter`s displays the current count and has
// buttons to increment and decrement the count.
impl<'a> Render<'a> for Counter {
  fn render(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
      use dodrio::builder::*;

      let bump = cx.bump;
      let count = bumpalo::format!(in bump, "{}", self.count);
      // let bump = cx.bump;
      
      dodrio!(bump,
          <div id="counter">
              <button 
                  class={style::button}
                  onclick={|root, vdom, _event| {
                  // Cast the root render component to a `Counter`, since
                  // we know that's what it is.
                  let counter = root.unwrap_mut::<Counter>();

                  // Increment the counter.
                  counter.increment();

                  // Since the count has updated, we should re-render the
                  // counter on the next animation frame.
                  vdom.schedule_render();
              }}>"+"</button>

              { vec![text(count.into_bump_str())] }

              <button onclick={|root, vdom, _event| {
                  // Same as above, but decrementing instead of incrementing.
                  root.unwrap_mut::<Counter>().decrement();
                  vdom.schedule_render();
              }}>"-"</button>
          </div>
      )
  }
}