function clear_chart(){
  $("#chartContainer").html('')
  $("#chartContainer_cost").html('') 
  $("#chartContainer_home").html('')   
}
//单个矿工 收益曲线图
function addSymbols(e){
	var suffixes = ["", "K", "M", "B"];

	var order = Math.max(Math.floor(Math.log(e.value) / Math.log(1000)), 0);
	if(order > suffixes.length - 1)
		order = suffixes.length - 1;

	var suffix = suffixes[order];
	return CanvasJS.formatNumber(e.value / Math.pow(1000, order)) + suffix;
}


function minerOneBenefitChart(){
  var chart = new CanvasJS.Chart("chartContainer", {
    animationEnabled: true,
    zoomEnabled: true,
    theme: "dark2",
    title:{
      text: "Benefit"
    },
    axisX:{
      title: "Year",
      valueFormatString: "####",
      interval: 2
    },
    axisY:{
      logarithmic: true, //change it to false
      title: "Profit in USD (Log)",
      prefix: "$",
      titleFontColor: "#6D78AD",
      lineColor: "#6D78AD",
      gridThickness: 0,
      lineThickness: 1,
      //includeZero: false,
      labelFormatter: addSymbols
    },
    axisY2:{
      title: "Profit in USD",
      prefix: "$",
      titleFontColor: "#51CDA0",
      logarithmic: false, //change it to true
      lineColor: "#51CDA0",
      gridThickness: 0,
      lineThickness: 1,
      labelFormatter: addSymbols
    },
    legend:{
      verticalAlign: "top",
      fontSize: 16,
      dockInsidePlotArea: true
    },
    data: [{
      type: "line",
      xValueFormatString: "####",
      yValueFormatString: "$#,##0.##",
      showInLegend: true,
      name: "Log Scale",
      dataPoints: [
        { x: 2001, y: 8000 },
        { x: 2002, y: 20000 },
        { x: 2003, y: 40000 },
        { x: 2004, y: 60000 },
        { x: 2005, y: 90000 },
        { x: 2006, y: 140000 },
        { x: 2007, y: 200000 },
        { x: 2008, y: 400000 },
        { x: 2009, y: 600000 },
        { x: 2010, y: 800000 },
        { x: 2011, y: 900000 },
        { x: 2012, y: 1400000 }, 
        { x: 2013, y: 2000000 }, 
        { x: 2014, y: 4000000 }, 
        { x: 2015, y: 6000000 }, 
        { x: 2016, y: 8000000 }, 
        { x: 2017, y: 9000000 }
      ]
    },
    {
      type: "line",
      xValueFormatString: "####",
      yValueFormatString: "$#,##0.##",
      axisYType: "secondary",
      showInLegend: true,
      name: "Linear Scale",
      dataPoints: [
        { x: 2001, y: 8000 },
        { x: 2002, y: 20000 },
        { x: 2003, y: 40000 },
        { x: 2004, y: 60000 },
        { x: 2005, y: 90000 },
        { x: 2006, y: 140000 },
        { x: 2007, y: 200000 },
        { x: 2008, y: 400000 },
        { x: 2009, y: 600000 },
        { x: 2010, y: 800000 },
        { x: 2011, y: 900000 },
        { x: 2012, y: 1400000 }, 
        { x: 2013, y: 2000000 }, 
        { x: 2014, y: 4000000 }, 
        { x: 2015, y: 6000000 }, 
        { x: 2016, y: 8000000 }, 
        { x: 2017, y: 9000000 }
      ]
    }]
  });
  chart.render();
}


//miner饼状图
function miner_pie(){
  var options = {
    title: {
      text: "Current Miner datas"
    },
    subtitles: [{
      text: "As of November, 2019"
    }],
    animationEnabled: true,
    data: [{
      type: "pie",
      startAngle: 40,
      toolTipContent: "<b>{label}</b>: {y}%",
      showInLegend: "true",
      legendText: "{label}",
      indexLabelFontSize: 16,
      indexLabel: "{label} - {y}%",
      dataPoints: [
        { y: 48.36, label: "miner1" },
        { y: 26.85, label: "miner2" },
        { y: 1.49, label: "miner3" },
        { y: 6.98, label: "miner4" },
      ]
    }]
  };
  $("#chartContainer").CanvasJSChart(options);
}

//回报率
function minerOneBenefitChartCost(){
  var totalVisitors = 883000;
  var visitorsData = {
    "New vs Returning Visitors": [{
      cursor: "pointer",
      explodeOnClick: false,
      innerRadius: "75%",
      legendMarkerType: "square",
      name: "New vs Returning Visitors",
      radius: "100%",
      showInLegend: true,
      startAngle: 90,
      type: "doughnut",
      dataPoints: [
        { y: 519960, name: "New Visitors", color: "#E7823A" },
        { y: 363040, name: "Returning Visitors", color: "#546BC1" }
      ]
    }],
    "New Visitors": [{
      color: "#E7823A",
      name: "New Visitors",
      type: "column",
      xValueFormatString: "MMM YYYY",
      dataPoints: [
        { x: new Date("1 Jan 2015"), y: 33000 },
        { x: new Date("1 Feb 2015"), y: 35960 },
        { x: new Date("1 Mar 2015"), y: 42160 },
        { x: new Date("1 Apr 2015"), y: 42240 },
        { x: new Date("1 May 2015"), y: 43200 },
        { x: new Date("1 Jun 2015"), y: 40600 },
        { x: new Date("1 Jul 2015"), y: 42560 },
        { x: new Date("1 Aug 2015"), y: 44280 },
        { x: new Date("1 Sep 2015"), y: 44800 },
        { x: new Date("1 Oct 2015"), y: 48720 },
        { x: new Date("1 Nov 2015"), y: 50840 },
        { x: new Date("1 Dec 2015"), y: 51600 }
      ]
    }],
    "Returning Visitors": [{
      color: "#546BC1",
      name: "Returning Visitors",
      type: "column",
      xValueFormatString: "MMM YYYY",
      dataPoints: [
        { x: new Date("1 Jan 2015"), y: 22000 },
        { x: new Date("1 Feb 2015"), y: 26040 },
        { x: new Date("1 Mar 2015"), y: 25840 },
        { x: new Date("1 Apr 2015"), y: 23760 },
        { x: new Date("1 May 2015"), y: 28800 },
        { x: new Date("1 Jun 2015"), y: 29400 },
        { x: new Date("1 Jul 2015"), y: 33440 },
        { x: new Date("1 Aug 2015"), y: 37720 },
        { x: new Date("1 Sep 2015"), y: 35200 },
        { x: new Date("1 Oct 2015"), y: 35280 },
        { x: new Date("1 Nov 2015"), y: 31160 },
        { x: new Date("1 Dec 2015"), y: 34400 }
      ]
    }]
  };


var newVSReturningVisitorsOptions = {
	animationEnabled: true,
	theme: "light2",
	title: {
		text: "New VS Returning Visitors"
	},
	subtitles: [{
		text: "Click on Any Segment to Drilldown",
		backgroundColor: "#2eacd1",
		fontSize: 16,
		fontColor: "white",
		padding: 5
	}],
	legend: {
		fontFamily: "calibri",
		fontSize: 14,
		itemTextFormatter: function (e) {
			return e.dataPoint.name + ": " + Math.round(e.dataPoint.y / totalVisitors * 100) + "%";  
		}
	},
	data: []
};

var visitorsDrilldownedChartOptions = {
	animationEnabled: true,
	theme: "light2",
	axisX: {
		labelFontColor: "#717171",
		lineColor: "#a2a2a2",
		tickColor: "#a2a2a2"
	},
	axisY: {
		gridThickness: 0,
		includeZero: false,
		labelFontColor: "#717171",
		lineColor: "#a2a2a2",
		tickColor: "#a2a2a2",
		lineThickness: 1
	},
	data: []
};
newVSReturningVisitorsOptions.data = visitorsData["New vs Returning Visitors"];
$("#chartContainer_cost").CanvasJSChart(newVSReturningVisitorsOptions);

}
// function visitorsChartDrilldownHandler(e) {
// 	e.chart.options = visitorsDrilldownedChartOptions;
// 	e.chart.options.data = visitorsData[e.dataPoint.name];
// 	e.chart.options.title = { text: e.dataPoint.name }
// 	e.chart.render();
// 	$("#backButton").toggleClass("invisible");
// }

// $("#backButton").click(function() { 
// 	$(this).toggleClass("invisible");
// 	newVSReturningVisitorsOptions.data = visitorsData["New vs Returning Visitors"];
// 	$("#chartContainer_cost").CanvasJSChart(newVSReturningVisitorsOptions);
// });

//home chart
function minerChart(){
  var options = {
    exportEnabled: true,
    animationEnabled: true,
    title:{
      text: "Units Sold VS Profit"
    },
    subtitles: [{
      text: "Click Legend to Hide or Unhide Data Series"
    }],
    axisX: {
      title: "States"
    },
    axisY: {
      title: "Units Sold",
      titleFontColor: "#4F81BC",
      lineColor: "#4F81BC",
      labelFontColor: "#4F81BC",
      tickColor: "#4F81BC",
      includeZero: false
    },
    axisY2: {
      title: "Profit in USD",
      titleFontColor: "#C0504E",
      lineColor: "#C0504E",
      labelFontColor: "#C0504E",
      tickColor: "#C0504E",
      includeZero: false
    },
    toolTip: {
      shared: true
    },
    legend: {
      cursor: "pointer",
    },
    data: [{
      type: "spline",
      name: "Units Sold",
      showInLegend: true,
      xValueFormatString: "MMM YYYY",
      yValueFormatString: "#,##0 Units",
      dataPoints: [
        { x: new Date(2016, 0, 1),  y: 120 },
        { x: new Date(2016, 1, 1), y: 135 },
        { x: new Date(2016, 2, 1), y: 144 },
        { x: new Date(2016, 3, 1),  y: 103 },
        { x: new Date(2016, 4, 1),  y: 93 },
        { x: new Date(2016, 5, 1),  y: 129 },
        { x: new Date(2016, 6, 1), y: 143 },
        { x: new Date(2016, 7, 1), y: 156 },
        { x: new Date(2016, 8, 1),  y: 122 },
        { x: new Date(2016, 9, 1),  y: 106 },
        { x: new Date(2016, 10, 1),  y: 137 },
        { x: new Date(2016, 11, 1), y: 142 }
      ]
    },
    {
      type: "spline",
      name: "Profit",
      axisYType: "secondary",
      showInLegend: true,
      xValueFormatString: "MMM YYYY",
      yValueFormatString: "$#,##0.#",
      dataPoints: [
        { x: new Date(2016, 0, 1),  y: 19034.5 },
        { x: new Date(2016, 1, 1), y: 20015 },
        { x: new Date(2016, 2, 1), y: 27342 },
        { x: new Date(2016, 3, 1),  y: 20088 },
        { x: new Date(2016, 4, 1),  y: 20234 },
        { x: new Date(2016, 5, 1),  y: 29034 },
        { x: new Date(2016, 6, 1), y: 30487 },
        { x: new Date(2016, 7, 1), y: 32523 },
        { x: new Date(2016, 8, 1),  y: 20234 },
        { x: new Date(2016, 9, 1),  y: 27234 },
        { x: new Date(2016, 10, 1),  y: 33548 },
        { x: new Date(2016, 11, 1), y: 32534 }
      ]
    }]
  };
  $("#chartContainer_home").CanvasJSChart(options);
}