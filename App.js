import React from 'react';
import { View, Text, Button } from 'react-native';

const App = () =>{
	return(
		<view style={{
			paddingLeft:10,
			paddingRight: 10,
		}}>
		<View style={{
			flex:1,
			flexDirection: 'row',
			justifyContent: 'space-between',
		}}>
		<View style={{
			flexDirection: 'row',
			justifyContent: 'flex-start',
		}}>
		<View>
		<Text style={{
			height: 50,
			fontSize: 25,
			backgroundColor: 'red'
		}}>Abu Ghalib</Text>
		</View>
		<View>
		<Text style={{
			height: 50,
			fontSize: 25,
			backgroundColor: 'green'
		}}>First</Text>
		</View>
		<View>
		<Text style={{
			height: 50,
			fontSize: 25,
			backgroundColor: 'yellow'
		}}>Second</Text>
		</View>
		
		</View>

		<View style={{
			flexDirection: 'row',
			justifyContent: 'flex-end',
		}}>
		<View>
		<Text style={{
			height: 50,
			fontSize: 25,
			backgroundColor: 'red'
		}}>Abugh</Text>
		</View>
		<View>
		<Text style={{
			height: 50,
			fontSize: 25,
			backgroundColor: 'green'
		}}>First</Text>
		</View>
		<View>
		<Text style={{
			height: 50,
			fontSize: 25,
			backgroundColor: 'yellow'
		}}>Second</Text>
		</View>
		</View>

		</View>

		<View style={{
			height:100,
			flex: 1,
			justifyContent: "center",
			
		}}>
		<Text style={{
			fontSize: 40,
		}}>Hi There I'm Abu Ghalib</Text>

		<Button style={{
			height: 20,
			width: 40,
		}}>Hello</Button>

		</View>

		</view>
	);
};

export default App;