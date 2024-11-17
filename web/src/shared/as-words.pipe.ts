import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'asWords',
  standalone: true
})
export class AsWordsPipe implements PipeTransform {

  transform(value: string, ...args: unknown[]): string {
    return value.split(/(?<![A-Z])(?=[A-Z])/).join(' ');
  }

}
