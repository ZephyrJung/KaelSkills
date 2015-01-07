package application;

import java.util.Random;

import javafx.application.Application;
import javafx.event.ActionEvent;
import javafx.event.EventHandler;
import javafx.geometry.Insets;
import javafx.geometry.Pos;
import javafx.scene.Scene;
import javafx.scene.control.Button;
import javafx.scene.control.Label;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;
import javafx.scene.input.KeyEvent;
import javafx.scene.layout.GridPane;
import javafx.scene.layout.HBox;
import javafx.scene.paint.Color;
import javafx.scene.text.Font;
import javafx.stage.Stage;

public class Main extends Application {
	private Label order=new Label();
	private Label aim=new Label();
	private Image image;
	private HBox hImage=new HBox();
	private HBox aImage=new HBox();
	private int count=0;
	private ImageView imageView=new ImageView();
	private long start;
	private long end;
	private int right;
	private Label time=new Label("-1秒");
	private GridPane grid = new GridPane();
	private EventHandler<KeyEvent> keyEvent;
	private String skills="BCDFGTVXYZ";
	@Override
	public void start(Stage primaryStage) throws Exception {
		// TODO Auto-generated method stub
		primaryStage.setTitle("Hello World");
		
		grid.setAlignment(Pos.TOP_LEFT);
		grid.setHgap(5);
		grid.setVgap(5);
		grid.setPadding(new Insets(15, 15, 15, 15));
		Scene scene = new Scene(grid, 450,150);
		
		aim.setFont(new Font("Comic Sans MS",30));	
		aim.setTextFill(Color.web("#FF0000"));
		time.setFont(new Font("Comic Sans MS",15));	
		time.setTextFill(Color.web("#FF0000"));
		order.setFont(new Font("Comic Sans MS",15));
		order.setWrapText(true);
		
		keyEvent=new EventHandler<KeyEvent>() {
		    public void handle(KeyEvent ke) { 	
		    	
		    	order.setText(order.getText()+ke.getText());
//		    	imageView=new ImageView(
//		    			new Image(getClass().getResourceAsStream(ke.getText().toUpperCase()+".png"))
//		    			);
		        if(order.getText().endsWith("r")&&count<10){
		        	if(count==0)
			    		start=System.currentTimeMillis();
		        	String temp=aim.getText().substring(count,count+1);
		        	
		        	if(checkSkill(order.getText(),temp)){
		        		image=new Image(getClass().getResourceAsStream(temp.toUpperCase()+".png"));
		        		right++;
		        	}else{
		        		image=new Image(getClass().getResourceAsStream("error.png"));
		        	}
		        	hImage.getChildren().add(new ImageView(image));
		        	order.setText("");	
		        	count++;
		        	if(count==10){
			    		end=System.currentTimeMillis();
			    		time.setText("  "+(end-start)/1000+"秒,正确率："+right*10+"%");
			    	}
		        }
		    }
		};
		Button start = new Button();
        start.setText("Start!");
        start.setOnAction(new EventHandler<ActionEvent>() {
            public void handle(ActionEvent event) {
            	aImage.getChildren().clear();
            	Random random=new Random();
            	String temp="";
            	for(int i=0;i<10;i++){
            		int index=random.nextInt(10);
            		temp+=skills.substring(0,1);
            		image=new Image(getClass().getResourceAsStream(skills.substring(0,1).toUpperCase()+".png"));
            		aImage.getChildren().add(new ImageView(image));
            	}
            	aim.setText(temp);
            	time.setText("-1秒");
            	hImage.getChildren().clear();
            	count=0;
            	right=0;
            	grid.setOnKeyPressed(keyEvent);
            }
        });    
        HBox hb=new HBox();
        hb.setAlignment(Pos.CENTER);
        hb.getChildren().add(start);
        hb.getChildren().add(time);
		grid.add(hb, 0, 0);
		grid.add(aImage, 0, 1);
		grid.add(hImage, 0, 2);
		Label note=new Label("点击start后，按下技能指令，以R结束。");
		grid.add(note, 0, 3);
		primaryStage.setScene(scene);
		primaryStage.show();
	}
	boolean checkSkill(String orders,String skill){
		int cice=0;
		int clight=0;
		int cfire=0;
		int ice=0;
		int light=0;
		int fire=0;
		char[] temp=orders.toCharArray();
		for(int i=0;i<temp.length;i++){
			if(temp[i]!='q'&&temp[i]!='w'&&temp[i]!='e'&&temp[i]!='r')
				return false;
			if(temp[i]=='q')
				cice++;
			if(temp[i]=='w')
				clight++;
			if(temp[i]=='e')
				cfire++;
		}
//		"BCDFGTVXYZ"
		switch(skills.indexOf(skill)){
			case 0:ice=1;light=1;fire=1;break;
			case 1:ice=0;light=3;fire=0;break;
			case 2:ice=0;light=1;fire=2;break;
			case 3:ice=1;light=0;fire=2;break;
			case 4:ice=2;light=0;fire=1;break;
			case 5:ice=0;light=0;fire=3;break;
			case 6:ice=2;light=1;fire=0;break;
			case 7:ice=1;light=2;fire=0;break;
			case 8:ice=3;light=0;fire=0;break;
			case 9:ice=0;light=2;fire=1;break;
		}
		if(cice==ice&&clight==light&&cfire==fire)
			return true;
		else
			return false;
	}
	public static void main(String[] args) {
		launch(args);
	}
}
