const eval_chart1 = (function () {
    const ctx = document.getElementById('chart1');
    let chartdata = [148, 150, 130, 170];
    const data = {
        labels: [
            'Food & beverages',
            'Groceries',
            'Gaming',
            'Trip & holiday',
        ],
        datasets: [{
            label: 'Total Expenses',
            data: chartdata,
            backgroundColor: [
                '#3B82F6',
                '#10B981',
                '#6366F1',
                '#F59E0B'
            ]
        }]
    };
    const config = {
        type: 'bar', //polarArea, bar, line, doughnut, pie, radar, scatter
        data: data,
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    position: 'bottom',
                },
            }
        }
    };
    const chart = new Chart(ctx, config);
    // 接收 Rust 发送的数据并更新图表
    (async function () {
        while (true) {
            const msg = await dioxus.recv();
            // console.log(msg);
            chart.data.datasets[0].data = msg;
            chart.update();
        }
    })();
});

setTimeout(() => {
    eval_chart1();
}, 100);