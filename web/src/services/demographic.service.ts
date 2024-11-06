import { HttpParams } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { environment } from '../environments/environment.development';
import { PlotlyGraph } from '../models/responses';
import { GridService } from './grid.service';
import { PlotService } from './plot.service';

const baseUrl = environment.api.base;

@Injectable({
  providedIn: 'root'
})
export class DemographicService {
  private gridService = inject(GridService);
  private plotService = inject(PlotService);

  constructor() {
  }

  getDemographics(): Observable<PlotlyGraph> {
    // TODO: This is the generic graph format for everything.
    // The deserialization nonsense can be pulled out into a utility method
    return this.plotService.getBarPlot('demographics');
  }

  plotADHDTypeByGender$(gender: string = ''): Observable<PlotlyGraph> {
    const params = new HttpParams().set("gender", gender);
    return this.plotService.getBarPlot('demographics', params);
  }


  plotADHDTypeByAgeRange$(gender: string = ''): Observable<PlotlyGraph> {
    return this.plotService.getBarPlot('demographics');
  }

}
