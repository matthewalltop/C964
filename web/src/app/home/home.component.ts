import { Component, CUSTOM_ELEMENTS_SCHEMA } from '@angular/core';
import { CommonModule } from '@angular/common';
import PlotlyJS from 'plotly.js-dist-min';
import { PlotlyModule } from 'angular-plotly.js';
import { MarkdownComponent, MarkdownModule, MarkdownService } from 'ngx-markdown';


PlotlyModule.plotlyjs = PlotlyJS;

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [CommonModule, MarkdownComponent],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent {
  ngOnInit() {
    console.log("Welcome home");
  }
}
