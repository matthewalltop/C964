import { HttpParams } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { PlotlyGraph, TableData } from '../models/responses';
import { GridService } from './grid.service';
import { PlotService } from './plot.service';


@Injectable({
  providedIn: 'root'
})
export class ExploreDataService {
  private gridService = inject(GridService);
  private plotService = inject(PlotService);

  plotDemographics$(subCategory: string, gender: string = '', with_controls: boolean = false): Observable<PlotlyGraph> {
    const params = new HttpParams().set('display', 'plot').set('sub_category', subCategory).set('gender', gender).set('with_controls', with_controls);
    return this.plotService.getBarPlot('demographics', params);
  }

  getDemographicsTableData$(subCategory: string, gender: string = '', with_controls: boolean = false): Observable<TableData> {
    const params = new HttpParams().set('display', 'table').set('sub_category', subCategory).set('gender', gender).set('with_controls', with_controls);
    return this.gridService.getTable('demographics', params);
  }

  plotMentalHealth$(subCategory: string, gender: string = '', with_controls: boolean = false): Observable<PlotlyGraph> {
    const params = new HttpParams().set('display', 'plot').set('sub_category', subCategory).set('gender', gender).set('with_controls', with_controls);
    return this.plotService.getBarPlot('mental_health', params);
  }

  getMentalHealthTableData$(subCategory: string, gender: string = '', with_controls: boolean = false): Observable<TableData> {
    const params = new HttpParams().set('display', 'table').set('sub_category', subCategory).set('gender', gender).set('with_controls', with_controls);
    return this.gridService.getTable('mental_health', params);
  }
}
