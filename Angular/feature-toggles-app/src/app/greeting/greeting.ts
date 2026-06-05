import { Component, input } from '@angular/core';
import { UpperCasePipe } from '@angular/common';

@Component({
  selector: 'app-greeting',
  imports: [UpperCasePipe],
  templateUrl: './greeting.html'
})

export class GreetingComponent{
  heading = input("Greetings");
  names = input<string[]>([]);
}
