import { Component } from '@angular/core';

// Stand-in for a "heavy" feature (imagine a charting lib, a rich editor, etc.).
// The whole point: this component's code should live in its OWN bundle chunk
// and only download when the user scrolls it into view via @defer (on viewport).
@Component({
  selector: 'app-heavy-panel',
  template: `
    <div class="heavy">
      <h4>📊 Heavy analytics panel</h4>
      <p>
        If you can read this, my JS chunk has already been fetched. In the
        Network tab you should see a separate <code>chunk-XXXX.js</code> that
        loaded only when this scrolled into view.
      </p>
    </div>
  `,
  styles: `
    .heavy {
      border: 2px dashed #888;
      padding: 1rem;
      margin-top: 500vh; /* pushed down so it starts BELOW the fold */
    }
  `,
})
export class HeavyPanel {}
