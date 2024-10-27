import { Component, inject } from '@angular/core';
import { map, Observable, switchMap, tap } from 'rxjs';
import { DemographicService } from '../../services/demographic.service';
import { CommonModule } from '@angular/common';


import * as PlotlyJS from 'plotly.js-dist-min';
import { PlotlyModule } from 'angular-plotly.js';
import { PlotlyGraph } from '../../models/responses';
PlotlyModule.plotlyjs = PlotlyJS;

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [CommonModule, PlotlyModule],
  providers: [DemographicService],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent {

  private demographicService = inject(DemographicService)
  public plotlyResponse$: Observable<PlotlyGraph> | undefined;


  ngOnInit() {
    this.plotlyResponse$ = this.demographicService.getDemographics();


  }


}
