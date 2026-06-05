import { Component, signal } from "@angular/core";
import { HeavyPanel } from "../heavy-panel/heavy-panel";

@Component({
    selector: 'app-feature-toggles',
    imports: [HeavyPanel],
    templateUrl: './feature-toggles.html'
})

export class FeatureToggles{
    showAdvanced = signal(false);
}