use std::io;
use std::process::Command;
use std::path::Path;

fn main() {
    loop {
        //清屏
        clearscreen::clear().expect("failed to clear screen");
        //显示主界面
        echo_choice_0();
        //声明“选项”变量
        let mut choice = String::new();
        //用户输入选项
        io::stdin().read_line(&mut choice).expect("输入无效");
        //遮蔽
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //模式选择
        match choice {
            1 => install_driver(),
            2 => {
                flash();
                break;
            }
            _ => {
                //清屏
                clearscreen::clear().expect("failed to clear screen(cserr0)");
                println!("输入无效,请重新输入");
                continue;
            }
        }
    }
}


//安装驱动
fn install_driver() {
    //清屏
    clearscreen::clear().expect("failed to clear screen");
    //打开Driver.exe
    opener::open("Driver.exe");
    return;
}

//刷入系统
fn flash() {
    //清屏
    clearscreen::clear().expect("failed to clear screen");
    //打印提示界面
    echo_tips_0();
    //声明“rec文件位置”变量
    let mut rec_path = String::new();

        println!("请把recovery镜像文件拖到框内,然后回车:");
        //用户输入文件位置
        io::stdin()
            .read_line(&mut rec_path)
            .expect("输入无效");


    //刷入rec
    Command::new("fastboot")
    .args(["flash", "recovery", &rec_path])
    .status()
    .expect("fastboot 执行失败");

    //进入rec模式
    Command::new("fastboot")
    .args(["boot", &rec_path])
    .status()
    .expect("fastboot 执行失败");

    println!("请检查手机是否进入了recovery模式");
    press_btn_continue::wait("按任意键继续").unwrap();

    //清屏
    clearscreen::clear().expect("failed to clear screen");
    //打印提示
    echo_choice_1();
    //声明系统文件位置变量
    let mut system_path = String::new();

        println!("请把刷机包文件拖到框内,然后回车:");
        //用户输入文件位置
        io::stdin()
            .read_line(&mut system_path)
            .expect("输入无效");


    //刷入系统
    Command::new("adb")
    .args(["sideload", &system_path])
    .status()
    .expect("adb 执行失败");


    //清屏
    clearscreen::clear().expect("failed to clear screen");
    //打印提示
    echo_tips_2();
    press_btn_continue::wait("Press any key to continue...").unwrap();
}




        //主界面
fn echo_choice_0() {
    println!("==========================================");
    println!("欢迎使用类原生系统刷入工具");
    println!("26-02-25");
    println!("==========================================");
    println!("");
    println!("==========================================");
    println!("1、安装驱动");
    println!("2、开始刷入");
    println!("==========================================");
    }

fn echo_choice_1() {

}

fn echo_tips_0() {
    println!("==========================================");
    println!("请确保你已经下载了刷机包和recovery文件");
    println!("请保证你已经解锁了bootloader");
    println!("==========================================");
    println!("");
    println!("==========================================");
    println!("请进入fastboot模式");
    println!("将手机关机,同时按住音量下和电源键开机，即可进入FASTBOOT");
    println!("==========================================");
}

fn echo_tips_1() {
    println!("==========================================");
    println!("接下来的操作可以采用触屏或者使用音量键和电源键来操作");
    println!("1.选择Factory reset");
    println!("2.选择Format data/factory reset");
    println!("3.依次完成Formatting /data, Formatting /cache, 和 Formatting /metadata");
    println!("4.点击屏幕左上角返回键返回到主界面");
    println!("5.选择Apply update");
    println!("6.选择Apply from ADB");
    println!("==========================================");
    println!("完成以上步骤即可点击任意键开始下一步");
}

fn echo_tips_2() {
    println!("==========================================");
    println!("刷入完成");
    println!("重新启动即可进入系统");
    println!("按任意键退出程序");
    println!("==========================================");
}






//按任意键继续
//press_btn_continue::wait("Press any key to continue...").unwrap();
