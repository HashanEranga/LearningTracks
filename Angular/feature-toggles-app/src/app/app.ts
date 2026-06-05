import { Component, signal } from '@angular/core';
import { FeatureToggles } from './feature-toggles/feature-toggles';
import { GreetingComponent } from './greeting/greeting';

@Component({
  selector: 'app-root',
  imports: [FeatureToggles, GreetingComponent],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  protected readonly title = signal('feature-toggles-app');
}
